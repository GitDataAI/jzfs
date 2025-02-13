use lib_entity::ColumnTrait;
use lib_entity::EntityTrait;
use lib_entity::QueryFilter;
use lib_entity::prelude::Uuid;
use lib_entity::repos::repos;
use lib_entity::users::users;
use serde::Serialize;

use crate::server::AppUserState;

#[derive(Serialize)]
pub struct UsersInfo {
    pub uid : Uuid,
    pub name : String,
    pub username : String,
    pub bio : Option<String>,
    pub pronouns : Option<String>,
    pub company : Option<String>,
    pub location : Option<String>,
    pub localtime : Option<String>,
    pub i18n : Option<String>,
    pub website : Vec<String>,
    pub orcid : Option<String>,
    pub social : Vec<String>,
    pub theme : String,
    pub pinned : Vec<Uuid>,
    pub followers : i32,
    pub following : i32,
    pub repository : i32,
    pub stars : i32,
    pub watching : i32,
    pub package : i32,
    pub release : i32,
    pub mentioned : bool,
    pub main_email : String,
    pub visible_email : bool,
    pub pro : bool,
    pub avatar_url : Option<String>,
    pub created : i64,
    pub updated : i64,
    pub hasused : i64,
    pub allow_use : bool,
    pub member : Vec<users::Model>,
    pub repos : Vec<repos::Model>,
    pub orgs : Vec<users::Model>,
}

impl AppUserState {
    pub async fn get_user_info(&self, uid : Uuid) -> anyhow::Result<UsersInfo> {
        let user = users::Entity::find_by_id(uid)
            .one(&self.read)
            .await?
            .ok_or_else(|| anyhow::anyhow!("user not found"))?;

        let mut info = UsersInfo {
            uid : user.uid,
            name : user.name,
            username : user.username,
            bio : user.bio,
            pronouns : user.pronouns,
            company : user.company,
            location : user.location,
            localtime : user.localtime,
            i18n : user.i18n,
            website : user.website,
            orcid : user.orcid,
            social : user.social,
            theme : user.theme,
            pinned : user.pinned,
            followers : user.followers,
            following : user.following,
            repository : user.repository,
            stars : user.stars,
            watching : user.watching,
            package : user.package,
            release : user.release,
            mentioned : user.mentioned,
            main_email : user.main_email,
            visible_email : user.visible_email,
            pro : user.pro,
            avatar_url : user.avatar_url,
            created : user.created,
            updated : user.updated,
            hasused : user.hasused,
            allow_use : user.allow_use,
            member : vec![],
            orgs : vec![],
            repos : vec![],
        };
        let member = users::Entity::find()
            .filter(users::Column::Uid.is_in(user.member))
            .all(&self.read)
            .await?;
        let repos = repos::Entity::find()
            .filter(repos::Column::Owner.eq(uid))
            .all(&self.read)
            .await?;
        let orgs = users::Entity::find()
            .filter(users::Column::Member.contains(uid))
            .all(&self.read)
            .await?;
        info.member = member;
        info.repos = repos;
        info.orgs = orgs;
        Ok(info)
    }
}
