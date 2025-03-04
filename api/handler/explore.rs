use actix_web::{web, Responder};
use crate::api::write::AppWrite;
use crate::services::recommend::hot::HotTimeParma;
use crate::services::AppState;
use crate::services::recommend::markplace::MarketplaceListParma;

pub async fn explore_repo_hot(
    app_state: web::Data<AppState>,
    parma: web::Json<HotTimeParma>,
)
    -> impl Responder
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


pub async fn marketplace(
    app_state: web::Data<AppState>,
    parma: web::Json<MarketplaceListParma>,
)
    -> impl Responder
{
    match app_state.marketplace_list(parma.0).await {
        Ok(data) => {
            AppWrite::ok(data)
        },
        Err(e) => {
            AppWrite::error(e.to_string())
        }
    }
}
