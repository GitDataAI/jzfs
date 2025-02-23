use captcha_rs::CaptchaBuilder;


pub struct CaptchaImage {
    pub base64: String,
    pub text: String,
}

impl Default for CaptchaImage {
    fn default() -> Self {
        Self::new()
    }
}

impl CaptchaImage {
    pub fn new() -> CaptchaImage {
        let captcha = CaptchaBuilder::new()
            .length(5)
            .width(110)
            .height(35)
            .dark_mode(false)
            .complexity(0) // min: 1, max: 10
            .compression(40) // min: 1, max: 99
            .build();
        let text = captcha.text.clone();
        let base64 = captcha.to_base64();
        Self { base64, text }
    }
}