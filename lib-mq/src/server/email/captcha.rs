use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct EmailCaptcha {
    pub email: String,
    pub code: String,
}

impl EmailCaptcha {
    pub fn generate_captcha(email: String) -> Self {
        let rand = rand::random::<u32>() % 100000;
        let code = rand.to_string();
        EmailCaptcha {
            email,
            code
        }
    }
}