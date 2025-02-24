use rand::Rng;

pub struct TokenUtils;

impl TokenUtils {
    pub fn generate_token() -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789";
        
        let mut rng = rand::rng();
        (0..64)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_length() {
        let token = TokenUtils::generate_token();
        assert_eq!(token.len(), 64);
    }

    #[test]
    fn test_token_characters() {
        let token = TokenUtils::generate_token();
        assert!(token.chars().all(|c| 
            c.is_ascii_digit() || 
            c.is_ascii_lowercase() || 
            c.is_ascii_uppercase()
        ));
    }
}
