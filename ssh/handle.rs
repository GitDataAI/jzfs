use crate::http::GIT_ROOT;
use crate::services::AppState;
use crate::model::users::{ssh, users};
use futures::future::FutureExt;
use russh::server::{Auth, Handle, Msg, Session};
use russh::{Channel, ChannelId, CryptoVec, Disconnect, MethodKind, MethodSet};
use std::collections::HashMap;
use std::io;
use std::process::{ExitStatus, Stdio};
use std::str::FromStr;
use russh::keys::PublicKey;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tokio::io::AsyncRead;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::ChildStdin;
use tokio::try_join;
use tracing::info;

pub struct SSHandle {
    pub app: AppState,
    pub stdin: HashMap<ChannelId, ChildStdin>,
    pub user: Option<users::Model>
}



impl russh::server::Handler for SSHandle {
    type Error = russh::Error;

    async fn auth_none(&mut self, _: &str) -> Result<Auth, Self::Error> {
        Ok(Auth::Reject {
            proceed_with_methods: None,
        })
    }
    async fn auth_password(&mut self, u: &str, p: &str) -> Result<Auth, Self::Error> {
        info!("auth_password: {:?} {:?}", u, p);
        let mut methods = MethodSet::empty();
        methods.push(MethodKind::PublicKey);
        Ok(Auth::Reject {
            proceed_with_methods: Some(methods),
        })
    }
    async fn channel_eof(&mut self, channel: ChannelId, _: &mut Session) -> Result<(), Self::Error> {
        let stdin = self.stdin.remove(&channel);
        if let Some(mut stdin) = stdin {
            stdin.shutdown().await.ok();
        }
        Ok(())
    }
    async fn channel_open_session(&mut self, _: Channel<Msg>, _: &mut Session) -> Result<bool, Self::Error> {
        Ok(true)
    }
    async fn data(&mut self, channel: ChannelId, data: &[u8], _: &mut Session) -> Result<(), Self::Error> {
        if let Some(stdin) = self.stdin.get_mut(&channel) {
            stdin.write_all(data).await.ok();
        }
        Ok(())
    }
    async fn auth_publickey(&mut self, user: &str, public_key: &PublicKey) -> Result<Auth, Self::Error> {
        if user != "git" {
            return Err(russh::Error::NotAuthenticated);
        }
        let public = public_key.to_string();
        if public.len() < 100 {
            return Err(russh::Error::NotAuthenticated);
        }
        dbg!(&public);
        let model = ssh::Entity::find()
            .filter(ssh::Column::Content.contains(&public))
            .one(&self.app.read)
            .await
            .map_err(|_| russh::Error::WrongServerSig)?
            .ok_or(russh::Error::NotAuthenticated)?;
        let user = match users::Entity::find()
            .filter(users::Column::Uid.eq(model.user_id))
            .one(&self.app.read)
            .await
            .map_err(|_| russh::Error::WrongServerSig)?
        {
            Some(user) => user,
            None => {
                return Err(russh::Error::NotAuthenticated);
            }
        };
        self.user = Some(user);
        Ok(Auth::Accept)
    }
    async fn exec_request(&mut self, channel: ChannelId, data: &[u8], session: &mut Session) -> Result<(), Self::Error> {
        let git_shell_cmd = match std::str::from_utf8(data){
            Ok(cmd) => cmd,
            Err(e) => {
                session.disconnect(Disconnect::ServiceNotAvailable, &e.to_string(), "").ok();
                return Err(russh::Error::Disconnect);
            }
        };
        let (service, path) =  if let Some(rec_pack_path) = git_shell_cmd.strip_prefix("git-receive-pack ") {
            (GitService::ReceivePack, strip_apostrophes(rec_pack_path))
        } else if let Some(upl_ref_path) = git_shell_cmd.strip_prefix("git-upload-pack ") {
            (GitService::UploadPack, strip_apostrophes(upl_ref_path))
        } else if let Some(upl_arc_path) = git_shell_cmd.strip_prefix("git-upload-archive ") {
            (GitService::UploadArchive, strip_apostrophes(upl_arc_path))
        } else {
            session.disconnect(Disconnect::ServiceNotAvailable,&format!("invalid git shell command: {git_shell_cmd:?}"),"" ).ok();
            return Err(russh::Error::Disconnect);
        };
        let (owner, repo) = if let Some((owner, repo)) = path.split_once('/') {
            (owner, repo)
        } else {
            session.disconnect(Disconnect::ServiceNotAvailable,&format!("invalid git shell command: {git_shell_cmd:?}"),"" ).ok();
            return Err(russh::Error::Disconnect);
        };
        let repo = match self.app.repo_info(owner.to_string(), repo.to_string()).await {
            Ok(repo) => repo,
            Err(e) => {
                session.disconnect(Disconnect::ByApplication, &e.to_string(), "").ok();
                return Err(russh::Error::Disconnect);
            }
        };
        let user = match self.user.clone() {
            Some(user) => user,
            None => {
                session.disconnect(Disconnect::ByApplication, "user not found", "").ok();
                return Err(russh::Error::Disconnect);
            }
        };
        let access = match self.app.user_access_owner_model(user.uid).await {
            Ok(access) => access,
            Err(e) => {
                session.disconnect(Disconnect::ByApplication, &e.to_string(), "").ok();
                return Err(russh::Error::Disconnect);
            }
        };
        if !access.iter().any(|x|x.repos.iter().any(|x|x.uid == repo.uid)){
            session.disconnect(Disconnect::ByApplication, "repository not found, please you have access clone", "").ok();
            return Err(russh::Error::Disconnect);
        }
        let path = format!("{}/{}/{}/", GIT_ROOT, repo.node_uid, repo.uid);
        let mut cmd = {
            let mut cmd = tokio::process::Command::new("git");
            cmd.arg("shell").arg("-c");
            match service {
                GitService::UploadPack => {
                    cmd.arg("git-upload-pack");
                }
                GitService::ReceivePack => {
                    cmd.arg("git-receive-pack");
                }
                GitService::UploadArchive => {
                    cmd.arg("git-upload-archive");
                }
            }
            cmd.current_dir(path);
            cmd
        };
        let mut shell = match cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(shell) => {
                session.channel_success(channel).ok();
                shell
            }
            Err(e) => {
                session.channel_failure(channel).ok();
                return Err(russh::Error::IO(e));
            }
        };
        
        let session_handle = session.handle();
        let stdin = shell.stdin.take().unwrap();
        self.stdin.insert(channel, stdin);
        let mut shell_stdout = shell.stdout.take().unwrap();
        let mut shell_stderr = shell.stderr.take().unwrap();

       let stdout_fut = forward(
            &session_handle,
            channel,
            &mut shell_stdout,
            move |handle, chan, data| async move { 
                handle.data(chan, data).await 
                    .map_err(|_| russh::Error::IO(io::Error::new(io::ErrorKind::Other, "stdout")))
            },
        ).fuse();
        
        let stderr_fut = forward(
            &session_handle,
            channel,
            &mut shell_stderr,
            move |handle, chan, data| async move {
                handle.extended_data(chan, 1, data).await
                    .map_err(|_| russh::Error::IO(io::Error::new(io::ErrorKind::Other, "stderr")))
            },
        ).fuse();

        tokio::pin!(stdout_fut, stderr_fut);

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
                Pipe::Stdout(result) => result?,
                Pipe::Stderr(result) => result?,
                Pipe::Exit(result) => {
                    let status = result?;
                    let status_code = status.code().unwrap_or(128) as u32;

                    let result = try_join!(stdout_fut, stderr_fut);
                    result.map_err(|_| russh::Error::IO(io::Error::new(io::ErrorKind::Other, "result stdout")))?;

                    session_handle.exit_status_request(channel, status_code).await.ok();
                    session_handle.eof(channel).await.ok();
                    session_handle.close(channel).await.ok();
                    break;
                }
            }
        }
        Ok(())
    }
}

async fn forward<'a, R, Fut, Fwd>(
    session_handle: &'a Handle,
    chan_id: ChannelId,
    r: &mut R,
    mut fwd: Fwd,
) -> Result<(), russh::Error>
where
    R: AsyncRead + Send + Unpin,
    Fut: Future<Output = Result<(),  russh::Error>> + 'a,
    Fwd: FnMut(&'a Handle, ChannelId, CryptoVec) -> Fut + 'a,
{
    const BUF_SIZE: usize = 1024 * 32;
    let mut buf = [0u8; BUF_SIZE];
    loop {
        let read = r.read(&mut buf).await?;
        if read == 0 {
            break;
        }
        if fwd(session_handle, chan_id, CryptoVec::from(&buf[..read])).await.is_err() {
            break;
        }
    }
    Ok(())
}

fn strip_apostrophes(s: &str) -> &str {
    if s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2 && !s[1..s.len() - 1].contains('\'')
    {
        &s[1..s.len() - 1]
    } else {
        s
    }
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