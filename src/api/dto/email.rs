use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmailCaptcha{
    pub email: String,
}
#[derive(Deserialize)]
pub struct EmailCaptchaCheck{
    pub email: String,
    pub code: String,
}
