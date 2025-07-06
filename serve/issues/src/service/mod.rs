use sea_orm::DatabaseConnection;
use cert::rpc::interface::CertInterFaceClient;
use cert::service::AppCertService;
use session::storage::RedisStorage;

#[derive(Clone)]
pub struct AppIssueService {
    db: DatabaseConnection,
    cache: RedisStorage,
    #[cfg(feature = "distributed")]
    cret: CertInterFaceClient,
    #[cfg(feature = "local")]
    cert: AppCertService
}
