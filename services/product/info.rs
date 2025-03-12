use std::io;
use sea_orm::EntityTrait;
use uuid::Uuid;
use crate::model::product::data_product;
use crate::services::AppState;
use crate::services::product::list::ProductList;

impl AppState {
    pub async fn product_info(&self, uid: Uuid) -> io::Result<ProductList> {
        let data = data_product::Entity::find_by_id(uid)
            .one(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Product not found"))?;
        Ok(ProductList {
            data: data.clone(),
            owner: self.user_info_by_uid(data.owner).await?,
            repo: self.repo_get_by_uid(data.repository_uid).await?,
        })
    }
}