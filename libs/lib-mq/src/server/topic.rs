use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub enum Topic {
    Email,
}

impl Topic {
    pub fn to_string(&self) -> String {
        match self {
            Topic::Email => "email".to_string(),
        }
    }
    pub fn try_from_string(s : String) -> Option<Topic> {
        match s.as_str() {
            "email" => Option::from(Topic::Email),
            _ => None,
        }
    }
}
