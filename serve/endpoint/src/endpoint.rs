#![allow(unused_imports)]

use cert::rpc::interface::CertInterFaceClient;
use cert::service::AppCertService;
use workhorse::rpc::proto::WorkHorseInterFaceClient;
use workhorse::service::AppWorkHorse;

#[derive(Clone)]
pub struct Endpoint {
    #[cfg(feature = "distributed")]
    pub cert: CertInterFaceClient,
    #[cfg(feature = "local")]
    pub cert: AppCertService,
    #[cfg(feature = "distributed")]
    pub workhorse: WorkHorseInterFaceClient,
    #[cfg(feature = "local")]
    pub workhorse: AppWorkHorse,
}

impl Endpoint {
    pub fn new_context(&self) -> tarpc::context::Context {
        tarpc::context::current()
    }
}


#[cfg(feature = "distributed")]
pub mod distributed;
#[cfg(feature = "distributed")]
pub use distributed::run;