use uuid::Uuid;

pub fn uuid_v4() -> Uuid {
    Uuid::new_v4()
}
