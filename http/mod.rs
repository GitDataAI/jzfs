
pub(crate) const GIT_ROOT: &str = "./data";

pub enum GitPack {
    UploadPack,
    ReceivePack
}



pub mod pack;
pub mod refs;


