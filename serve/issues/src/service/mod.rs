use sea_orm::DatabaseConnection;
use session::storage::RedisStorage;

#[derive(Clone)]
pub struct AppIssueService {
    db: DatabaseConnection,
    cache: RedisStorage,
}
