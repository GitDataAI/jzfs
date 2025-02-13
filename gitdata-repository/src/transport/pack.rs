use std::io;
use std::io::Cursor;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use async_fn_stream::fn_stream;
use bytes::Bytes;
use flate2::bufread::GzDecoder;
use futures_core::Stream;

use crate::transport::GitServiceType;
use crate::transport::Transport;

impl Transport {
    pub async fn pack(
        &self,
        path : &str,
        service : GitServiceType,
        version : Option<String>,
        gzip : bool,
        payload : Bytes,
        stateless : bool,
    ) -> io::Result<impl Stream<Item = Result<Bytes, Error>> + use<>> {
        let mut cmd = Command::new("git");
        cmd.arg(service.to_string());
        if stateless {
            cmd.arg("--stateless-rpc");
        }
        cmd.arg(".");
        if !version.is_some() {
            cmd.env("GIT_PROTOCOL", version.unwrap_or("".to_string()));
        }
        let path = PathBuf::from(path).join("/.git");
        if !PathBuf::from(path.clone()).exists() {
            return Err(Error::new(
                io::ErrorKind::Other,
                "Repository Bare NotFount".to_string(),
            ));
        }
        cmd.stderr(Stdio::piped());
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.current_dir(path);
        let mut git_process = match cmd.spawn() {
            Ok(process) => process,
            Err(e) => {
                return Err(Error::new(
                    io::ErrorKind::Other,
                    format!("Error running command {}", e),
                ));
            }
        };
        let mut stdin = git_process.stdin.take().unwrap();
        let mut stdout = git_process.stdout.take().unwrap();
        let bytes = if gzip {
            let mut decoder = GzDecoder::new(Cursor::new(payload));
            let mut decoded_data = Vec::new();
            if let Err(e) = io::copy(&mut decoder, &mut decoded_data) {
                return Err(Error::new(
                    io::ErrorKind::Other,
                    format!("Error running command {}", e),
                ));
            }
            decoded_data
        } else {
            payload.to_vec()
        };
        if let Err(e) = stdin.write_all(&bytes) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error running command {}", e),
            ));
        }
        drop(stdin);
        Ok(fn_stream(move |emitter| async move {
            let mut buffer = [0; 8192];
            loop {
                match stdout.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => {
                        emitter
                            .emit(Ok(Bytes::copy_from_slice(&buffer[0..n])))
                            .await;
                    }
                    Err(e) => {
                        emitter.emit(Err(Error::new(io::ErrorKind::Other, e))).await;
                        break;
                    }
                }
            }
        }))
    }
}
