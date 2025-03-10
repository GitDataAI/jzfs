use crate::model::product::data_product;
use crate::services::AppState;
use std::io;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ProductListParam {
    pub page: u64,
    pub limit: u64,
    pub order: String,
    pub search: Option<String>,
}


impl AppState {
    pub async fn product_list(&self, parma: ProductListParam) -> io::Result<Vec<data_product::Model>> {
        let mut query = data_product::Entity::find()
            .order_by_asc(data_product::Column::CreatedAt)
            .filter(
                data_product::Column::Name.contains(parma.search.clone().unwrap_or("".to_string()))
                    .or(data_product::Column::Description.contains(parma.search.unwrap_or("".to_string())))
            )
            .limit(parma.limit)
            .offset((parma.page - 1) * parma.limit)
            .all(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        if parma.order == "desc" {
            query.reverse();
        }
        Ok(query)
    }
}