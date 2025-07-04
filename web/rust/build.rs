use std::env;
use std::env::current_dir;
use std::process::Command;

fn main() {
    let env = env::var("PATH").unwrap();
    #[cfg(not(target_os = "windows"))]
    let path = env.split(":").collect::<Vec<_>>();
    #[cfg(target_os = "windows")]
    let path = env.split(";").collect::<Vec<_>>();
    let pnpm = path
        .iter()
        .find(|x|{
            let path = std::path::Path::new(x).join("pnpm").exists();
            path
        })
        .expect("pnpm not found");
    println!("cargo:rustc-env=PATH={}", pnpm);
    #[cfg(not(target_os = "windows"))]
    let exec = "pnpm";
    #[cfg(target_os = "windows")]
    let exec = "pnpm.cmd";
    let version = Command::new(exec)
        .args(["--version"])
        .current_dir(pnpm)
        .output()
        .expect("failed to execute process")
        .stdout;
    println!("cargo:rustc-env=PNPM_VERSION={}", String::from_utf8_lossy(&version));
    let web_dir = current_dir().unwrap().join("web");
    if !web_dir.exists() {
        panic!("web directory not found");
    }
    let out = Command::new(exec)
        .args([
            "install",
            "-w"
        ])
        .current_dir(web_dir.clone())
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&out.stdout));

    let build = Command::new(exec)
        .args([
            "run",
            "build:all"
        ])
        .current_dir(web_dir)
        .output()
        .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&build.stdout));
}