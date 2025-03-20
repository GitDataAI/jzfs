use uuid::{ContextV7, Uuid};

pub fn uuid_v7() -> Uuid {
    let context = ContextV7::new();
    let timestamp = chrono::Utc::now().timestamp();
    Uuid::new_v7(uuid::Timestamp::from_unix(&context, timestamp as u64, 0))
}
