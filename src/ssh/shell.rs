use std::fmt::Display;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GitService {
    UploadPack,
    ReceivePack,
    UploadArchive,
}

impl Display for GitService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            GitService::UploadPack => "upload-pack".to_string(),
            GitService::ReceivePack => "receive-pack".to_string(),
            GitService::UploadArchive => "upload-archive".to_string(),
        };
        write!(f, "{}", str)
    }
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
pub fn strip_apostrophes(s: &str) -> &str {
    if s.starts_with('\'') && s.ends_with('\'') && s.len() >= 2 && !s[1..s.len() - 1].contains('\'')
    {
        &s[1..s.len() - 1]
    } else {
        s
    }
}
impl GitService {
    pub fn parse_git_shell_cmd(git_shell_cmd: &str) -> (Self, &str) {
        if let Some(rec_pack_path) = git_shell_cmd.strip_prefix("git-receive-pack ") {
            (GitService::ReceivePack, strip_apostrophes(rec_pack_path))
        } else if let Some(upl_ref_path) = git_shell_cmd.strip_prefix("git-upload-pack ") {
            (GitService::UploadPack, strip_apostrophes(upl_ref_path))
        } else if let Some(upl_arc_path) = git_shell_cmd.strip_prefix("git-upload-archive ") {
            (GitService::UploadArchive, strip_apostrophes(upl_arc_path))
        } else {
            panic!("invalid git shell command: {git_shell_cmd:?}");
        }
    }
}
pub fn reconstruct_shell_cmd(service: GitService, path: &str) -> String {
    let service_str = match service {
        GitService::ReceivePack => "receive-pack",
        GitService::UploadPack => "upload-pack",
        GitService::UploadArchive => "upload-archive",
    };

    format!("git-{service_str} '{path}'")
}

pub struct RequiredRepoPermissions {
    pub read: bool,
    pub write: bool,
}

impl RequiredRepoPermissions {
    pub fn for_service(service: GitService) -> Self {
        match service {
            GitService::UploadPack => Self {
                read: true,
                write: false,
            },
            GitService::ReceivePack => Self {
                read: true,
                write: true,
            },
            GitService::UploadArchive => Self {
                read: true,
                write: false,
            },
        }
    }
    pub fn for_i32(access: i32) -> Self {
        match access {
            0 => Self {
                read: true,
                write: false,
            },
            1 => Self {
                read: true,
                write: true,
            },
            2 => Self {
                read: true,
                write: true,
            },
            3 => Self {
                read: true,
                write: true,
            },
            _ => Self {
                read: false,
                write: false,
            }
        }
    }
}

