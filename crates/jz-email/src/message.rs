use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub enum EmailMessage {
    Captcha { email: String, code: String },
}
