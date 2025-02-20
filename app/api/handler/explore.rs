use crate::app::api::write::AppWrite;
use crate::app::services::recommend::hot::HotTimeParma;
use crate::app::services::AppState;
use poem::{handler, web, IntoResponse};


#[handler]
pub async fn explore_repo_hot(
    app_state: web::Data<&AppState>,
    parma: web::Json<HotTimeParma>,
)
    -> impl IntoResponse
{
    match app_state.hot_repo(parma.0).await {
        Ok(data) => {
            AppWrite::ok(data)
        },
        Err(e) => {
            AppWrite::error(e.to_string())
        }
    }
}
