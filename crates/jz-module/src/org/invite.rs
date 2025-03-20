use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateInvite {
    pub user_name: String,
    pub user_uid: Uuid,
}