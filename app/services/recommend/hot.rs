use crate::app::services::AppState;
use crate::model::repository::repository;
use crate::model::statistics::statistics_repo;
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use uuid::Uuid;
use crate::model::origin::organization;
// Click * 0.05 + Fork * 0.3 + Watch * 0.3 + Star * 0.8

#[derive(Deserialize,Serialize,Clone,Debug)]
pub  struct HotTimeParma {
    pub start: HotTime,
    pub end: HotTime,
    pub limit: i64,
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct HotTime {
    years: i32,
    month: i32,
    day: i32,
}

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct HotRepo {
    pub complex: f64,
    pub click: i64,
    pub fork: i64,
    pub star: i64,
    pub owner: String,
    pub model: repository::Model
}


impl AppState {
    pub async fn hot_repo(&self, parma: HotTimeParma) -> io::Result<Vec<HotRepo>> {
        let mut hot_repo = vec![];
        let start = parma.start;
        let end = parma.end;
        let mut statistics: HashMap<Uuid, HotRepo> = HashMap::new();
        let mut condition = Condition::all();
        if start.years == end.years {
            condition = condition.add(statistics_repo::Column::Years.eq(end.years))
        } else {  
            condition = condition.add(
                Condition::any()
                    .add(statistics_repo::Column::Years.between(start.years, end.years))
                    .add(statistics_repo::Column::Years.between(end.years, start.years))
            );
        }
        if start.month == end.month {
            condition = condition.add(statistics_repo::Column::Mount.eq(end.month))
        } else {  
            condition = condition.add(
                Condition::any()
                    .add(statistics_repo::Column::Mount.between(start.month, end.month))
                    .add(statistics_repo::Column::Mount.between(end.month, start.month))
            );
        }
        if start.day == end.day {
            condition = condition.add(statistics_repo::Column::Days.eq(end.day))
        } else {  
            condition = condition.add(
                Condition::any()
                    .add(statistics_repo::Column::Days.between(start.day, end.day))
                    .add(statistics_repo::Column::Days.between(end.day, start.day))
            );
        }
        
        let models = statistics_repo::Entity::find()
            .filter(condition)
            .all(&self.read)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        for (count,idx) in models.iter().enumerate() {
            if count > parma.limit as usize {
                break;
            }
            let repo = repository::Entity::find_by_id(idx.repo_uid)
                .one(&self.read)
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
           
            if let Some(repo) = repo {
                let owner = self.user_info_by_uid(repo.owner_id).await;
                let name = if let Ok(e) = owner {
                    e.name
                } else {
                    let group = organization::Entity::find_by_id(idx.uid).one(&self.read).await;
                    if let Ok(Some(e)) = group {
                        e.name
                    }else {
                        continue
                    }
                };
                if let Some(v) = statistics.get_mut(&repo.uid) {
                    match idx.rtype.as_str() {
                        "click" => v.click += idx.count,
                        "fork" => v.fork += idx.count,
                        "star" => v.star += idx.count,
                        _ => {}
                    }
                } else {
                    let mut v = HotRepo {
                        complex: 1.,
                        click: 1,
                        fork: 1,
                        star: 1,
                        owner: name,
                        model: repo.clone(),
                    };
                    match idx.rtype.as_str() {
                        "click" => v.click += idx.count,
                        "fork" => v.fork += idx.count,
                        "star" => v.star += idx.count,
                        _ => {}
                    }
                    statistics.insert(repo.uid, v);
                }
            }
        }
        for (_, mut v) in statistics {
            v.complex = v.click as f64 * 0.05  + v.fork  as f64 * 0.3 + v.star  as f64 * 0.8;
            hot_repo.push(v);
        }
        Ok(hot_repo)
        
    }
}