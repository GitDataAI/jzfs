use std::path::Path;
use russh_keys::PrivateKey;
use russh_keys::signature::rand_core::OsRng;

pub mod server;
pub mod handler;
pub mod shell;


pub const ENV_REPO_UID: &str = "ENV_REPO_UID";
pub const ENV_USER_UID: &str = "ENV_USER_UID";


pub fn init_git_ssh_backend() -> PrivateKey {
    if !Path::new("./ed25519").exists(){
        let key = PrivateKey::random(&mut OsRng, russh_keys::Algorithm::Ed25519).unwrap();
        key.write_openssh_file(Path::new("./ed25519"), Default::default()).unwrap();
        key
    }else { 
        let key = PrivateKey::read_openssh_file(Path::new("./ed25519")).unwrap();
        key
    }
    
}