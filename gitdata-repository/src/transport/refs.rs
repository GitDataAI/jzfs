use std::io::Error;
use std::path::PathBuf;
use std::process::Command;

use tokio::io;

use crate::transport::GitServiceType;
use crate::transport::Transport;

impl Transport {
    pub async fn refs(
        &self,
        path : &str,
        service : GitServiceType,
        version : Option<String>,
    ) -> io::Result<String> {
        let path = PathBuf::from(path).join(".git");
        if !path.exists() {
            return Err(Error::new(io::ErrorKind::Other, "Path does not exist"));
        }
        let mut cmd = Command::new("git");
        cmd.arg(service.to_string());
        cmd.arg("--stateless-rpc");
        cmd.arg("--advertise-refs");
        cmd.arg(".");
        cmd.current_dir(path);
        if !version.is_some() {
            cmd.env("GIT_PROTOCOL", version.unwrap_or("".to_string()));
        }
        let output = match cmd.output() {
            Ok(output) => output,
            Err(e) => {
                return Err(Error::new(
                    io::ErrorKind::Other,
                    format!("Error running command {}", e),
                ));
            }
        };
        if !output.status.success() {
            return Err(Error::new(io::ErrorKind::Other, "Error running command"));
        }
        Ok(String::from_utf8(output.stdout).unwrap_or("".to_string()))
    }
}
