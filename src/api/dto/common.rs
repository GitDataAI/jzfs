use serde::Deserialize;
use utoipa::ToSchema;

#[allow(unused)]
#[derive(Deserialize, ToSchema)]
pub struct ListOption<T>{
    pub offset: i64,
    pub limit: i64,
    pub order: String,
    pub filter: T,
}