use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct UsersFollowLink {
    target_uid: Uuid,
}
