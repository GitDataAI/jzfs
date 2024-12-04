use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct EmailCaptcha{
    pub email: String,
}
#[derive(Deserialize, ToSchema)]
pub struct EmailCaptchaCheck{
    pub email: String,
    pub code: String,
}
