use sea_orm::ColumnTrait;
use std::io;
use chrono::{Datelike, Utc};
use lazy_static::lazy_static;
use sea_orm::{EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;
use uuid::Uuid;
use crate::app::services::AppState;
use crate::model::statistics::statistics_repo;


lazy_static!{
    pub static ref STAR:String = "star".to_string();
    pub static ref FORK:String = "fork".to_string();
    pub static ref WATCH:String = "watch".to_string();
    pub static ref CLICK:String = "click".to_string();
}

impl AppState {
    pub async fn statistics_repo(&self, uid: Uuid, rtype: String) -> io::Result<()> {
        let time = Utc::now();
        let year = time.year() as i64;
        let month = time.month() as i64;
        let day = time.day() as i64;
        if let Some(model) = statistics_repo::Entity::find()
            .filter(
                statistics_repo::Column::Uid.eq(uid)
                    .and(statistics_repo::Column::Years.eq(year))
                    .and(statistics_repo::Column::Mount.eq(month))
                    .and(statistics_repo::Column::Days.eq(day))
                    .and(statistics_repo::Column::Rtype.eq(rtype.clone()))
                ,
            )
            .one(&self.write)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        {
            statistics_repo::Entity::update_many()
                .col_expr(statistics_repo::Column::Count, Expr::col(statistics_repo::Column::Count).add(1))
                .filter(statistics_repo::Column::Uid.eq(uid))
                .filter(statistics_repo::Column::Years.eq(year))
                .filter(statistics_repo::Column::Mount.eq(month))
                .filter(statistics_repo::Column::Days.eq(day))
                .filter(statistics_repo::Column::Rtype.eq(rtype.clone()))
                .exec(&self.write)
                .await
                .map(|_| ())
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        }else { 
            statistics_repo::Entity::insert(statistics_repo::ActiveModel {
                uid: Set(Uuid::new_v4()),
                repo_uid: Set(uid),
                years: Set(year),
                mount: Set(month),
                days: Set(day),
                count: Set(1),
                rtype: Set(rtype),
            })
                .exec(&self.write)
                .await
                .map(|_| ())
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        }
        Ok(())
    }
}