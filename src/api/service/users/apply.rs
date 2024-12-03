use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;
use crate::api::dto::users::UserApply;
use crate::api::service::users::UserService;
use crate::metadata::model::users::users;

impl UserService {
    pub async fn apply(&self, dto: UserApply) -> anyhow::Result<Uuid>{
        let username = dto.username;
        let email = dto.email;
        let passwd = dto.password;
        let model = users::ActiveModel{
            uid: Set(Uuid::new_v4()),
            name: Set(username.clone()),
            username: Set(username),
            email: Set(email),
            passwd: Set(passwd),
            status: Set(1),
            team: Set(vec![]),
            repo: Set(vec![]),
            project: Set(vec![]),
            issue: Set(vec![]),
            pr: Set(vec![]),
            commit: Set(vec![]),
            tag: Set(vec![]),
            star: Set(vec![]),
            follow: Set(vec![]),
            pro: Set(false),
            theme: Set("default".to_string()),
            localtime: Set("UTC".to_string()),
            timezone: Set("UTC".to_string()),
            company: Set("".to_string()),
            website: Set(vec![]),
            sex: Default::default(),
            description: Default::default(),
            avatar: Set(Option::from("".to_string())),
            created_at: Set(time::OffsetDateTime::now_utc()),
            updated_at: Set(time::OffsetDateTime::now_utc()),
            public_email: Set(false),
            phone: Default::default(),
            lastlogin: Set(time::OffsetDateTime::now_utc()),
        };
        match model.insert(&self.db).await{
            Ok(x) => {
                Ok(x.uid)
            },
            Err(e) => return Err(anyhow::anyhow!(e))
        }         
    }
}