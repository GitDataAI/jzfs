use chrono::Utc;
use sea_orm::DeleteResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeleteOption {
    pub rows_affected: u64,
    pub time: i64,
}

impl From<DeleteResult> for DeleteOption {
    fn from(result: DeleteResult) -> Self {
        DeleteOption {
            rows_affected: result.rows_affected,
            time: Utc::now().timestamp(),
        }
    }
}
