
pub mod endpoint;

pub fn check_feature() {
    if cfg!(feature = "local") && cfg!(feature = "distributed") {
        tracing::error!("local and distributed only one option can be selected");
        std::process::exit(1);
    };
    tracing::info!("The system will start in {} mode ", if cfg!(feature = "local") {"local"} else {"distributed"});
}

pub mod cert;
pub mod routes;
pub mod utils;