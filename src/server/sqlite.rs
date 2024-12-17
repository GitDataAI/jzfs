use sea_orm::DatabaseConnection;
use tokio::sync::OnceCell;

pub static SQLITEDB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn init_sqlite(){
    SQLITEDB.get_or_init(|| async {
        tracing::info!("Initializing SQLite...");
        let db = sea_orm::Database::connect("sqlite::memory:").await.unwrap();
        db
    }).await;
}