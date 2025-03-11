use crate::model::product::data_product;
use crate::services::AppState;
use std::io;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::repository::repository;
use crate::model::users::users;

#[derive(Deserialize)]
pub struct ProductListParam {
    pub page: u64,
    pub limit: u64,
    pub order: String,
    pub search: Option<String>,
}

#[derive(Serialize,Deserialize)]
pub struct ProductList {
    pub data: data_product::Model,
    pub owner: users::Model,
    pub repo: repository::Model
}


impl AppState {
    pub async fn product_list(&self, parma: ProductListParam) -> io::Result<Vec<ProductList>> {
        let mut query = data_product::Entity::find()
            .order_by_asc(data_product::Column::CreatedAt)
            .filter(
                if parma.search.clone().is_some() && parma.search.clone().unwrap_or("".to_string()) != "All" {
                    Condition::any()
                        .add(data_product::Column::Name.contains(parma.search.clone().unwrap_or("".to_string())))
                        .add(data_product::Column::Type.contains(parma.search.clone().unwrap_or("".to_string())))
                } else {
                    Condition::all()
                }
            )
            .limit(parma.limit)
            .offset((parma.page - 1) * parma.limit)
            .all(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        if parma.order == "desc" {
            query.reverse();
        }
        let mut result = vec![];
        for i in query {
            let owner = users::Entity::find_by_id(i.owner)
                .one(&self.read)
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
                .ok_or(io::Error::new(io::ErrorKind::Other, "owner not found"))?;
            let repo = repository::Entity::find_by_id(i.repository_uid)
                .one(&self.read)
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
                .ok_or(io::Error::new(io::ErrorKind::Other, "repo not found"))?;
            result.push(ProductList {
                data: i,
                owner,
                repo
            });
        }
        Ok(result)
    }
    pub async fn product_owner(&self, uid: Uuid) -> io::Result<Vec<ProductList>> {
        let query = data_product::Entity::find()
            .order_by_asc(data_product::Column::CreatedAt)
            .filter(data_product::Column::Owner.eq(uid))
            .all(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        let mut result = vec![];
        for i in query {
            let owner = users::Entity::find_by_id(i.owner)
                .one(&self.read)
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
                .ok_or(io::Error::new(io::ErrorKind::Other, "owner not found"))?;
            let repo = repository::Entity::find_by_id(i.repository_uid)
                .one(&self.read)
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
                .ok_or(io::Error::new(io::ErrorKind::Other, "repo not found"))?;
            result.push(ProductList {
                data: i,
                owner,
                repo
            });
        }
        Ok(result)
    }
}