use crate::metadata::model::users::UsersData;
use crate::metadata::service::MetaService;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::{ExitStatus, Stdio};
use std::sync::Arc;
use russh::server::{Auth, Handle, Handler, Msg, Session};
use russh::{Channel, ChannelId, CryptoVec, MethodSet};
use russh_keys::PublicKey;
use sea_orm::prelude::async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use tokio::process::ChildStdin;
use tracing::{error, info};
use crate::ssh::shell::{strip_apostrophes, GitService};

use futures::future::FutureExt;
use crate::ROOT_PATH;

pub struct SshHandler {
    pub server: MetaService,
    pub peer_addr: Option<SocketAddr>,
    pub users_data: Option<UsersData>,
    pub channel: HashMap<ChannelId, ChildStdin>,
}


impl SshHandler {
    pub fn new(server: MetaService, peer_addr: Option<SocketAddr>) -> Self {
        Self {
            server,
            peer_addr,
            users_data: None,
            channel: Default::default(),
        }
    }

    // Helper function to send stdin data
    async fn send_stdin(
        &mut self,
        channel: ChannelId,
        data: &[u8],
    ) -> Result<(), anyhow::Error> {
        if let Some(stdin) = self.channel.get_mut(&channel) {
            stdin.write_all(data).await?;
            stdin.flush().await?;
        } else {
            error!("Channel {channel:?} not found for stdin forwarding");
            return Err(anyhow::anyhow!("Channel {channel:?} does not exist"));
        }
        Ok(())
    }

    async fn parse_git_command(&self, git_shell_cmd: &str) -> Result<(GitService, String), anyhow::Error> {
        let (service, path) = if let Some(rec_pack_path) = git_shell_cmd.strip_prefix("git-receive-pack ") {
            (GitService::ReceivePack, strip_apostrophes(rec_pack_path))
        } else if let Some(upl_ref_path) = git_shell_cmd.strip_prefix("git-upload-pack ") {
            (GitService::UploadPack, strip_apostrophes(upl_ref_path))
        } else if let Some(upl_arc_path) = git_shell_cmd.strip_prefix("git-upload-archive ") {
            (GitService::UploadArchive, strip_apostrophes(upl_arc_path))
        } else {
            return Err(anyhow::anyhow!("Invalid git shell command: {git_shell_cmd}"));
        };

        // Ensure path is properly formatted
        let path_segments: Vec<_> = path.trim_start_matches('/').split('/').collect();
        if path_segments.len() < 2 {
            return Err(anyhow::anyhow!("Invalid repository path: {path}"));
        }

        let owner = path_segments[0].to_string();
        let repo_name = path_segments[1].split('.').next().unwrap_or_default().to_string();

        if repo_name.is_empty() {
            return Err(anyhow::anyhow!("Invalid repository name in path: {path}"));
        }
        let repo = self.server.repo_service().owner_name_by_uid(owner.clone(), repo_name).await;
        if repo.is_err() {
            return Err(anyhow::anyhow!("Repository not found: {path}"));
        }
        let repo_path = repo?.to_string();
        Ok((service, repo_path))
    }
    async fn forward<R, F, Fut>(
        session_handle: Arc<Handle>, 
        chan_id: ChannelId,
        mut reader: R,
        mut forward_fn: F,
    ) -> anyhow::Result<()>
    where
        R: AsyncRead + Unpin,
        F: FnMut(Arc<Handle>, ChannelId, CryptoVec) -> Fut,
        Fut: std::future::Future<Output = Result<(), CryptoVec>>,
    {
        const BUF_SIZE: usize = 32 * 1024;
        let mut buf = [0u8; BUF_SIZE];
        while let Ok(read) = reader.read(&mut buf).await {
            info!("Read {read} bytes from reader");
            if read == 0 {
                break;
            }
            if forward_fn(session_handle.clone(), chan_id, CryptoVec::from_slice(&buf[..read]))
                .await
                .is_err()
            {
                break;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Handler for SshHandler {
    type Error = anyhow::Error;

    async fn auth_none(&mut self, _: &str) -> Result<Auth, Self::Error> {
        Ok(Auth::Reject {
            proceed_with_methods: Some(MethodSet::PUBLICKEY),
        })
    }

    async fn auth_password(&mut self, _: &str, _: &str) -> Result<Auth, Self::Error> {
        Ok(Auth::Reject {
            proceed_with_methods: Some(MethodSet::PUBLICKEY),
        })
    }

    async fn auth_publickey(&mut self, user: &str, _: &PublicKey) -> Result<Auth, Self::Error> {
        if user != "git" {
            return Ok(Auth::Reject { proceed_with_methods: None });
        }
        // TODO Auth in Database
        Ok(Auth::Accept)
    }

    async fn channel_open_session(&mut self, _: Channel<Msg>, _: &mut Session) -> Result<bool, Self::Error> {
        Ok(true)
    }

    async fn data(&mut self, channel: ChannelId, data: &[u8], _: &mut Session) -> Result<(), Self::Error> {
        self.send_stdin(channel, data).await
    }

    async fn exec_request(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let git_shell_cmd = std::str::from_utf8(data).map_err(|_| anyhow::anyhow!("Invalid UTF-8 input"))?;
        info!("Executing command: {git_shell_cmd}");

        let (service, path) = self.parse_git_command(git_shell_cmd).await?;

        let cmd = tokio::process::Command::new("git")
            .arg(service.to_string())
            .arg(".")
            .current_dir(PathBuf::from(ROOT_PATH).join(&path))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        let mut shell = cmd.map_err(|e| anyhow::anyhow!("Failed to start git process: {e}"))?;

        let stdin = shell.stdin.take().ok_or_else(|| anyhow::anyhow!("Failed to open stdin"))?;
        self.channel.insert(channel, stdin);

        let stdout = shell.stdout.take().unwrap();
        let stderr = shell.stderr.take().unwrap();

        let session_handle = Arc::new(session.handle());
        let _fut = {
            let session_handle = session_handle.clone(); 
            async move {
                let stdout_fut = Self::forward(
                    session_handle.clone(),
                    channel,
                    stdout,
                    |handle, chan, data| async move { handle.data(chan, data).await },
                )
                    .fuse();

                let stderr_fut = Self::forward(
                    session_handle.clone(),
                    channel,
                    stderr,
                    |handle, chan, data| async move { handle.extended_data(chan, 1, data).await },
                )
                    .fuse();

                tokio::pin!(stdout_fut, stderr_fut);

                loop {
                    enum Pipe {
                        Stdout(Result<(), anyhow::Error>),
                        Stderr(Result<(), anyhow::Error>),
                        Exit(std::io::Result<ExitStatus>),
                    }

                    let result = tokio::select! {
                        result = shell.wait() => Pipe::Exit(result),
                        result = &mut stdout_fut => Pipe::Stdout(result),
                        result = &mut stderr_fut => Pipe::Stderr(result),
                    };

                    match result {
                        Pipe::Stdout(result) => {
                            result?;
                        }
                        Pipe::Stderr(result) => {
                            result?;
                        }
                        Pipe::Exit(result) => {
                            let status = result?;
                            let status_code = status.code().unwrap_or(128) as u32;
                            session_handle.exit_status_request(channel, status_code).await.ok();
                            session_handle.eof(channel).await.ok();
                            session_handle.close(channel).await.ok();
                            break;
                        }
                    }
                }
                Ok::<(), anyhow::Error>(())
            }
        };
        tokio::spawn(_fut);
        Ok(())
    }
}

