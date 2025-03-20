use serde::Deserialize;
use serde::de::DeserializeOwned;

#[derive(Deserialize)]
pub struct RequestContext<T> {
    pub unix: i32,
    pub inner: T,
    pub device: String,
}

impl<T: DeserializeOwned> RequestContext<T> {
    pub fn unix(&self) -> i32 {
        self.unix
    }
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

pub type RequestBody<T> = actix_web::web::Json<RequestContext<T>>;
