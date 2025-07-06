#![allow(unused_imports)]

use cert::rpc::interface::CertInterFaceClient;
use cert::service::AppCertService;
use workhorse::rpc::proto::WorkHorseInterFaceClient;

#[derive(Clone)]
#[cfg(feature = "distributed")]
pub struct Endpoint {
    pub cert: CertInterFaceClient,
    pub workhorse: WorkHorseInterFaceClient,
}


impl Endpoint {
    pub fn new_context(&self) -> tarpc::context::Context {
        tarpc::context::current()
    }
}