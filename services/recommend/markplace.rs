use std::io;
use serde::{Deserialize, Serialize};
use crate::model::product::data_product;
use crate::services::AppState;
use sea_orm::*;
use serde_json::json;
use crate::model::origin::organization;
use crate::model::users::users;

#[derive(Deserialize,Serialize)]
pub struct MarketplaceListParma {
    pub page: u64,
    pub limit: u64,
}

impl AppState {
    pub async fn marketplace_list(&self, parma: MarketplaceListParma) -> io::Result<Vec<serde_json::Value>>{
        let data = data_product::Entity::find()
            .limit(parma.limit)
            .offset(parma.page)
            .all(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        
        let mut map = Vec::new();
        for idx in data {
            let owner = if let Some(x) = users::Entity::find()
                .filter(users::Column::Uid.eq(idx.owner))
                .one(&self.read)
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
            {
                json!(x)
            } else if let Some(x) = organization::Entity::find()
                .filter(
                    organization::Column::Uid.eq(idx.owner)
                )
                .one(&self.read)
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
            {
                json!(x)
            }else { 
                json!({
                    "uid": idx.owner,
                })
            };
            map.push(json!({
                "owner": owner,
                "data": idx,
            }));
        }
        
        Ok(map)
    }
}