use jzfs::config::file::Config;

#[tokio::main]
async fn main() {
    let model = std::fs::read("./config/config.toml").unwrap();
    let cfg = toml::from_str::<Config>(std::str::from_utf8(&model).unwrap()).unwrap();
    let ml = toml::to_string(&cfg).unwrap();
    print!("{}", ml)
}