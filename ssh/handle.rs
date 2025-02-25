use crate::http::GIT_ROOT;
use crate::services::AppState;
use crate::model::users::{ssh, users};
use russh::server::{Auth, Handle, Msg, Session};
use russh::{Channel, ChannelId, CryptoVec, Disconnect, MethodKind, MethodSet};
use std::collections::{HashMap, HashSet};
use std::io;
use std::process::{ExitStatus, Stdio};
use std::str::FromStr;
use russh::keys::PublicKey;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tokio::io::AsyncRead;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::ChildStdin;
use tokio::sync::mpsc::Sender;
use tracing::error;

pub struct SSHandle {
    pub app: AppState,
    pub stdin: HashMap<ChannelId, ChildStdin>,
    pub user: Option<users::Model>,
    pub eof: HashMap<ChannelId, Sender<bool>>
}

impl SSHandle {
    pub fn new(app: AppState) -> Self {
        Self  {
            app,
            stdin: HashMap::new(),
            user: None,
            eof: HashMap::new()
        }
    }
}

impl russh::server::Handler for SSHandle {
    type Error = russh::Error;

    async fn auth_none(&mut self, _: &str) -> Result<Auth, Self::Error> {
        Ok(Auth::Reject {
            proceed_with_methods: None,
        })
    }

    async fn auth_password(&mut self, _: &str, _: &str) -> Result<Auth, Self::Error> {
        let mut methods = MethodSet::empty();
        methods.push(MethodKind::PublicKey);
        Ok(Auth::Reject {
            proceed_with_methods: Some(methods),
        })
    }

    async fn auth_publickey(&mut self, user: &str, public_key: &PublicKey) -> Result<Auth, Self::Error> {
        if user != "git" {
            return Err(russh::Error::NotAuthenticated);
        }

        let public = public_key.to_string();
        if public.len() < 100 {
            return Err(russh::Error::NotAuthenticated);
        }

        let model = ssh::Entity::find()
            .filter(ssh::Column::Content.contains(&public))
            .one(&self.app.read)
            .await
            .map_err(|e| {
                error!("Database query failed: {}", e);
                russh::Error::WrongServerSig
            })?
            .ok_or_else(|| {
                russh::Error::NotAuthenticated
            })?;

        let user = users::Entity::find()
            .filter(users::Column::Uid.eq(model.user_id))
            .one(&self.app.read)
            .await
            .map_err(|e| {
                error!("User query failed: {}", e);
                russh::Error::WrongServerSig
            })?
            .ok_or_else(|| {
                russh::Error::NotAuthenticated
            })?;

        self.user = Some(user);
        Ok(Auth::Accept)
    }

    async fn channel_close(&mut self, channel: ChannelId, _: &mut Session) -> Result<(), Self::Error> {
        if let Some(mut stdin) = self.stdin.remove(&channel) {
            let _ = stdin.shutdown().await;
        }
        Ok(())
    }
    async fn channel_eof(&mut self, channel: ChannelId, _: &mut Session) -> Result<(), Self::Error> {
        if let Some(mut stdin) = self.stdin.remove(&channel) {
            let _ = stdin.shutdown().await;
        }
        if let Some(eof) = self.eof.get_mut(&channel){
            eof.send(true).await.ok();
        }
        Ok(())
    }

    async fn channel_open_session(&mut self, _: Channel<Msg>, _: &mut Session) -> Result<bool, Self::Error> {
        Ok(true)
    }

    async fn data(&mut self, channel: ChannelId, data: &[u8], _: &mut Session) -> Result<(), Self::Error> {
        if let Some(stdin) = self.stdin.get_mut(&channel) {
            let _ = stdin.write_all(data).await;
            stdin.flush().await.ok();
        }
        Ok(())
    }
    async fn exec_request(&mut self, channel_id: ChannelId, data: &[u8], session: &mut Session) -> Result<(), Self::Error> {
        let git_shell_cmd = match std::str::from_utf8(data) {
            Ok(cmd) => cmd.trim(),
            Err(e) => {
                let msg = "Invalid command encoding";
                error!("{}: {}", msg, e);
                session.disconnect(Disconnect::ServiceNotAvailable, msg, "").ok();
                return Err(russh::Error::Disconnect);
            }
        };

        let (service, path) = match parse_git_command(git_shell_cmd) {
            Some((s, p)) => (s, p),
            None => {
                let msg = format!("Invalid git command: {}", git_shell_cmd);
                error!("{}", msg);
                session.disconnect(Disconnect::ServiceNotAvailable, &msg, "").ok();
                return Err(russh::Error::Disconnect);
            }
        };

        let (owner, repo) = match parse_repo_path(path) {
            Some(pair) => pair,
            None => {
                let msg = format!("Invalid repository path: {}", path);
                error!("{}", msg);
                session.disconnect(Disconnect::ServiceNotAvailable, &msg, "").ok();
                return Err(russh::Error::Disconnect);
            }
        };
        let repo = repo.replace(".git", "");
        let repo = match self.app.repo_info(owner.to_string(), repo.to_string()).await {
            Ok(repo) => repo,
            Err(e) => {
                error!("Repository lookup failed: {}", e);
                session.disconnect(Disconnect::ByApplication, "Repository not found", "").ok();
                return Err(russh::Error::Disconnect);
            }
        };

        let user = match &self.user {
            Some(user) => user,
            None => {
                error!("User not resolved after authentication");
                session.disconnect(Disconnect::ByApplication, "Authentication error", "").ok();
                return Err(russh::Error::Disconnect);
            }
        };

        let access = match self.app.user_access_owner_model(user.uid).await {
            Ok(access) => access,
            Err(e) => {
                error!("Access check failed: {}", e);
                session.disconnect(Disconnect::ByApplication, "Access check failed", "").ok();
                return Err(russh::Error::Disconnect);
            }
        };

        let allowed_repos: HashSet<_> = access.iter()
            .flat_map(|a| &a.repos)
            .map(|r| r.uid)
            .collect();

        if !allowed_repos.contains(&repo.uid) {
            error!("Access denied to repo {} for user {}", repo.uid, user.uid);
            session.disconnect(Disconnect::ByApplication, "Access denied", "").ok();
            return Err(russh::Error::Disconnect);
        }
        let path = format!("{}/{}/{}/", GIT_ROOT, repo.node_uid, repo.uid);
        let mut cmd = build_git_command(service, &path);
        let mut shell = match cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(shell) => {
                session.channel_success(channel_id).ok();
                shell
            }
            Err(e) => {
                error!("Process spawn failed: {}", e);
                session.channel_failure(channel_id).ok();
                return Err(russh::Error::IO(e));
            }
        };
        let session_handle = session.handle();
        let stdin = shell.stdin.take().unwrap();
        self.stdin.insert(channel_id, stdin);
        let mut shell_stdout = shell.stdout.take().unwrap();
        let mut shell_stderr = shell.stderr.take().unwrap();

        let (eof_tx, mut eof_rx) = tokio::sync::mpsc::channel::<bool>(10);
        self.eof.insert(channel_id, eof_tx);

        let fut = async move {
            async fn forward<'a, R, Fut, Fwd>(
                session_handle: &'a Handle,
                chan_id: ChannelId,
                r: &mut R,
                mut fwd: Fwd,
            ) -> Result<(), russh::Error>
            where
                R: AsyncRead + Send + Unpin,
                Fut: Future<Output = Result<(), CryptoVec>> + 'a,
                Fwd: FnMut(&'a Handle, ChannelId, CryptoVec) -> Fut,
            {
                const BUF_SIZE: usize = 1024 * 32;

                let mut buf = [0u8; BUF_SIZE];
                loop {
                    let read = r.read(&mut buf).await?;

                    if read == 0 {
                        break;
                    }

                    if fwd(session_handle, chan_id, CryptoVec::from_slice(&buf[..read]))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }

                Ok(())
            }

            use futures::future::FutureExt;

            let stdout_fut = forward(
                &session_handle,
                channel_id,
                &mut shell_stdout,
                |handle, chan, data| async move { handle.data(chan, data).await },
            )
                .fuse();

            tokio::pin!(stdout_fut);

            let stderr_fut = forward(
                &session_handle,
                channel_id,
                &mut shell_stderr,
                |handle, chan, data| async move {
                    // SSH_EXTENDED_DATA_STDERR = 1
                    handle.extended_data(chan, 1, data).await
                },
            )
                .fuse();

            tokio::pin!(stderr_fut);
            loop {
                enum Pipe {
                    Stdout(Result<(), russh::Error>),
                    Stderr(Result<(), russh::Error>),
                    Exit(io::Result<ExitStatus>),
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
                        // let _ = tokio::join!(stdout_fut, stderr_fut);
                        // stdout_fut.await?;
                        // stderr_fut.await?;
                        while let Some(eof) = eof_rx.recv().await {
                            if eof {
                                break;
                            }
                        }
                        let status_code = status.code().unwrap_or(128) as u32;
                        let _ = session_handle
                            .exit_status_request(channel_id, status_code)
                            .await;
                        let _ = session_handle.eof(channel_id).await;
                        let _ = session_handle.close(channel_id).await;
                        break;
                    }
                }
            }


            Ok::<(), russh::Error>(())
        };

        tokio::spawn(fut);
        Ok(())
    }
}

fn parse_git_command(cmd: &str) -> Option<(GitService, &str)> {
    let (svc, path) = match cmd.split_once(' ') {
        Some(("git-receive-pack", path)) => (GitService::ReceivePack, path),
        Some(("git-upload-pack", path)) => (GitService::UploadPack, path),
        Some(("git-upload-archive", path)) => (GitService::UploadArchive, path),
        _ => return None,
    };
    Some((svc, strip_apostrophes(path)))
}

fn parse_repo_path(path: &str) -> Option<(&str, &str)> {
    let path = path.trim_matches('/');
    let mut parts = path.splitn(2, '/');
    match (parts.next(), parts.next()) {
        (Some(owner), Some(repo)) if !owner.is_empty() && !repo.is_empty() => Some((owner, repo)),
        _ => None,
    }
}

fn build_git_command(service: GitService, path: &str) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new("git");
    // cmd.arg("shell").arg("-c").current_dir(path);
    cmd.current_dir(path);
    match service {
        GitService::UploadPack => cmd.arg("upload-pack"),
        GitService::ReceivePack => cmd.arg("receive-pack"),
        GitService::UploadArchive => cmd.arg("upload-archive"),
    };
    cmd
        .arg(".")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_NO_REPLACE_OBJECTS", "1");
    cmd
}

fn strip_apostrophes(s: &str) -> &str {
    s.trim_matches('\'')
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GitService {
    UploadPack,
    ReceivePack,
    UploadArchive,
}

impl FromStr for GitService {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "upload-pack" => Ok(Self::UploadPack),
            "receive-pack" => Ok(Self::ReceivePack),
            "upload-archive" => Ok(Self::UploadArchive),
            _ => Err(()),
        }
    }
}
