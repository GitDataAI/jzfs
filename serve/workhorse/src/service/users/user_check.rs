use sea_orm::*;
use authd::users;
use cert::schema::{result_ok_with_data, AppResult};
use crate::schema::users::UserCheckParam;
use crate::service::AppWorkHorse;

impl AppWorkHorse {
    pub async fn service_user_check(&self, param: UserCheckParam) -> AppResult<i32> {
        let username = if let Some(username) = param.username {
            if users::Entity::find()
                .filter(users::Column::Username.eq(username))
                .all(&self.db)
                .await
                .unwrap_or(vec![])
                .len() == 0 {
                0
            } else { 
                1
            }
        } else { 
            0
        };
        let email = if let Some(email) = param.email {
            if users::Entity::find()
                .filter(users::Column::Email.eq(email))
                .all(&self.db)
                .await
                .unwrap_or(vec![])
                .len() == 0 {
                0
            } else { 
                2
            }
        } else { 
            0
        };
        result_ok_with_data(email + username)
    }
}