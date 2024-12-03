use sea_orm::EntityTrait;
use uuid::Uuid;
use crate::api::service::check::CheckService;
use crate::metadata::model::groups::group;
use crate::metadata::model::repos::repo;
use crate::metadata::model::teams::teams;
use crate::metadata::model::users::users;

impl CheckService {
    pub async fn check_exits_user(&self, uid: Uuid) -> bool {
        let user = users::Entity::find_by_id(uid).one(&self.db).await;
        match user {
            Ok(user) => user.is_some(),
            Err(_) => false,
        }
    }
    pub async fn check_exits_group(&self, uid: Uuid) -> bool {
        let user = group::Entity::find_by_id(uid).one(&self.db).await;
        match user {
            Ok(user) => user.is_some(),
            Err(_) => false,
        }
    }
    pub async fn check_exits_repo(&self, uid: Uuid) -> bool {
        let user = repo::Entity::find_by_id(uid).one(&self.db).await;
        match user {
            Ok(user) => user.is_some(),
            Err(_) => false,
        }
    }
    pub async fn check_exits_team(&self, uid: Uuid) -> bool {
        let user = teams::Entity::find_by_id(uid).one(&self.db).await;
        match user {
            Ok(user) => user.is_some(),
            Err(_) => false,
        }
    }
    
}