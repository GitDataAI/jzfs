#[derive(thiserror::Error, Debug)]
pub enum RusshServerError {
    #[error("russh error: {0}")]
    Russh(#[from] russh::Error),
    #[error("russh-keys error: {0}")]
    RusshKeys(#[from] russh_keys::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("other error: {0}")]
    Other(#[from] anyhow::Error),
}
