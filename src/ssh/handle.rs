use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use anyhow::anyhow;
use log::info;
use russh::{Channel, ChannelId, MethodSet};
use russh::server::{Auth, Handler, Msg, Session};
use russh_keys::PublicKey;
use sea_orm::prelude::async_trait::async_trait;
use tokio::io::AsyncWriteExt;
use tokio::process::ChildStdin;
use uuid::Uuid;
use crate::api::service::Service;
use crate::ssh::error::RusshServerError;
use crate::ssh::server::RusshServer;
use crate::store::inode::RepoFileTrait;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GitService {
    UploadPack,
    ReceivePack,
    UploadArchive,
    Unknown,
}

fn strip_apostrophes(s: &str) -> &str {
    if s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2 && !s[1..s.len() - 1].contains('\'')
    {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

pub fn parse_git_shell_cmd(git_shell_cmd: &str) -> (GitService, &str) {
    if let Some(rec_pack_path) = git_shell_cmd.strip_prefix("git-receive-pack ") {
        (GitService::ReceivePack, strip_apostrophes(rec_pack_path))
    } else if let Some(upl_ref_path) = git_shell_cmd.strip_prefix("git-upload-pack ") {
        (GitService::UploadPack, strip_apostrophes(upl_ref_path))
    } else if let Some(upl_arc_path) = git_shell_cmd.strip_prefix("git-upload-archive ") {
        (GitService::UploadArchive, strip_apostrophes(upl_arc_path))
    } else {
        (GitService::Unknown, "")
    }
}


pub struct RusshServerHandler {
    fs_service: HashMap<ChannelId, Arc<Mutex<Box<dyn RepoFileTrait + 'static + Send>>>>,
    db_service: Service,
    peer_addr: Option<SocketAddr>,
    user_id: Option<Uuid>,
    stdin: HashMap<ChannelId, ChildStdin>,
}

impl RusshServerHandler {
    pub(crate) fn new(server: &mut RusshServer, peer_addr: Option<SocketAddr>) -> Self {
        Self {
            fs_service: HashMap::new(),
            db_service: server.service.clone(),
            peer_addr,
            user_id: None,
            stdin: HashMap::new(),
        }
    }
    async fn send_stdin(
        &mut self,
        channel_id: ChannelId,
        data: &[u8],
    ) -> Result<(), RusshServerError> {
        if let Some(stdin) = self.stdin.get_mut(&channel_id) {
            stdin.write_all(data).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl Handler for RusshServerHandler {
    type Error = RusshServerError;
    async fn auth_none(&mut self, user: &str) -> Result<Auth, Self::Error> {
        if user != "git" {
            return Ok(
                Auth::Reject {
                    proceed_with_methods: None,
                },
            );
        }
        Ok(
            Auth::Reject {
                proceed_with_methods: Some(MethodSet::PUBLICKEY),
            },
        )
    }
    async fn auth_password(&mut self, user: &str, _password: &str) -> Result<Auth, Self::Error> {
        if user != "git" {
            return Ok(
                Auth::Reject {
                    proceed_with_methods: None,
                },
            );
        }
        Ok(Auth::Reject {
            proceed_with_methods: None
        })
    }
    async fn auth_publickey(&mut self, user: &str, public_key: &PublicKey) -> Result<Auth, Self::Error> {
        if user != "git"{
            return Ok(
                Auth::Reject {
                    proceed_with_methods: None,
                },
            );
        }
        // let user_id = self.db_service.users.get_users_by_pubkey(public_key).await;
        // if user_id.is_err(){
        //     return Ok(Auth::Reject {
        //         proceed_with_methods: None
        //     })
        // }     
        // self.user_id = Some(user_id?);
        Ok(Auth::Accept)
    }
    async fn shell_request(&mut self, channel: ChannelId, session: &mut Session) -> Result<(), Self::Error> {
        self.send_stdin(channel, b"Hello").await.ok();
        session.channel_success(channel).ok();
        Ok(())
        
    }
    async fn channel_eof(&mut self, channel: ChannelId, _session: &mut Session) -> Result<(), Self::Error> {
        let stdin = self.stdin.remove(&channel);
        let _ = self.fs_service.remove(&channel).is_some();
        if let Some(mut stdin) = stdin {
            stdin.flush().await?;
            stdin.shutdown().await?;
        }
        Ok(())
    }
    async fn channel_open_session(&mut self, _channel: Channel<Msg>, _session: &mut Session) -> Result<bool, Self::Error> {
        
        Ok(true)
    }
    async fn data(&mut self, channel: ChannelId, data: &[u8], _session: &mut Session) -> Result<(), Self::Error> {
        self.send_stdin(channel, data).await?;
        Ok(())
    }
    async fn exec_request(&mut self, _channel: ChannelId, data: &[u8], session: &mut Session) -> Result<(), Self::Error> {
        let git_shell_cmd = std::str::from_utf8(data);
        let handle = session.config();
        
        println!("exec_request: data: {:?}", git_shell_cmd);
        if git_shell_cmd.is_err() {
            return Err(Self::Error::Other(anyhow!("Invalid utf8 string")))
        }
        let (service,path) = parse_git_shell_cmd(git_shell_cmd.unwrap());
        info!("exec_request: service: {:?}, path: {:?}", service, path);
        let paths = path
        .split("/")
        .filter(|s| !s.is_empty())
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
        if paths.len() < 2 {
            return Err(Self::Error::Other(anyhow!("Invalid path")));
        }
        let (owner, repo_name) = (paths[0].clone(), paths[1].clone());
        println!("owner: {:?}, repo_name: {:?}", owner, repo_name);
        if owner != "git" {
            return Err(Self::Error::Other(anyhow!("Service Not Open the Server")));
        }
        match service{
            GitService::UploadPack => {
                
            },
            GitService::ReceivePack => {
                
            },
            GitService::UploadArchive => {
                
            },
            _ => {
                return Err(Self::Error::Other(anyhow!("Service Not Open the Server")));
            }
        }
        Ok(())
    }
}


impl Drop for RusshServerHandler {
    fn drop(&mut self) {
        
    }
}