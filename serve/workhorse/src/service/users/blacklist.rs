use crate::schema::users::{UserBlackListItem, UserFollowCount, UserFollowItem, UsersFollowLink};
use crate::service::AppWorkHorse;
use authd::users;
use cert::schema::{result_error_with_msg, result_error_with_msg_data, result_ok, result_ok_with_data, AppResult};
use chrono::Utc;
use sea_orm::*;
use userd::{user_blacklist, user_follow};
use uuid::Uuid;

impl AppWorkHorse {
    pub async fn service_user_black(&self, param: UserBlackListItem) -> AppResult<()> {
        let follow = user_blacklist::ActiveModel {
            uid: Set(Uuid::now_v7()),
            user_id: Set(param.user_id),
            target_id: Set(param.target_id),
            created_at: Set(Utc::now().naive_local()),
            description: Set(param.description),
        }
            .insert(&self.db)
            .await;
        match follow {
            Ok(_) => {
                result_ok()
            }
            Err(error) => {
                result_error_with_msg(error.to_string())
            }
        }
    }
    pub async fn service_user_unblack(&self, param: UserBlackListItem) -> AppResult<()> {
        match user_follow::Entity::delete_many()
            .filter(user_blacklist::Column::UserId.eq(param.user_id))
            .filter(user_blacklist::Column::TargetId.eq(param.target_id))
            .exec(&self.db)
            .await {
            Ok(_) => {
                result_ok()
            }
            Err(error) => {
                result_error_with_msg(error.to_string())
            }
        }
    }

    pub async fn service_user_black_list(&self, user_uid: Uuid) -> AppResult<Vec<UserFollowItem>> {
        let follow = user_blacklist::Entity::find()
            .filter(user_blacklist::Column::UserId.eq(user_uid))
            .all(&self.db)
            .await;
        match follow {
            Ok(result) => {
                let user = users::Entity::find()
                    .filter(users::Column::Uid.is_in(result.iter().map(|x|x.target_id).collect::<Vec<Uuid>>()))
                    .all(&self.db)
                    .await;
                match user {
                    Ok(users) => {
                        let mut res = vec![];
                        for user_item in users {
                            if let Some(follow_item) = result.iter().find(|x|x.target_id == user_item.uid) {
                                res.push(UserFollowItem {
                                    username: user_item.username,
                                    uid: user_item.uid,
                                    description: user_item.description,
                                    avatar: user_item.avatar,
                                    created_at: follow_item.created_at,
                                    special: false,
                                });
                            }
                        }
                        result_ok_with_data(res)
                    }
                    Err(err) => {
                        result_error_with_msg_data(err.to_string())
                    }
                }
            }
            Err(err) => {
                result_error_with_msg_data(err.to_string())
            }
        }
    }
    pub async fn service_user_black_count(&self, user_id: Uuid) -> AppResult<u64> {
        let black_count = user_blacklist::Entity::find()
            .filter(user_follow::Column::UserId.eq(user_id))
            .count(&self.db)
            .await
            .unwrap_or(0);
        result_ok_with_data(black_count)
    }
}
