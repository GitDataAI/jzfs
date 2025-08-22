pub mod http;
pub mod ssh;

pub enum GitPack {
    UploadPack,
    ReceivePack,
}
