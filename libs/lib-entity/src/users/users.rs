use sea_orm::ActiveValue::Set;
// use crate::teams::teamsus;
use sea_orm::*;
use serde::Deserialize;
use serde::Serialize;
use sha256::Sha256Digest;
use uuid::Uuid;

use crate::repos::repos;
use crate::sea_query::TableCreateStatement;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
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
    #[serde(skip_serializing)]
    pub password : String,
    pub avatar_url : Option<String>,
    pub created : i64,
    pub updated : i64,
    pub hasused : i64,
    pub allow_use : bool,
    pub member : Vec<Uuid>,
}
impl ActiveModelBehavior for ActiveModel {}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    EmailsModels,
    SshKeysModels,
    TokenKeysModels,
    StarModels,
    FollowerModel,
    TwoFactor,
    // TeamsModel,
    RepoModel,
}
impl Related<super::email::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EmailsModels.def()
    }
}
impl Related<super::ssh_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SshKeysModels.def()
    }
}
impl Related<super::token_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TokenKeysModels.def()
    }
}
impl Related<super::star::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StarModels.def()
    }
}
impl Related<super::follower::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FollowerModel.def()
    }
}
impl Related<super::two_factor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TwoFactor.def()
    }
}
// impl Related<teamsus::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::TeamsModel.def()
//     }
// }
impl Related<repos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RepoModel.def()
    }
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::EmailsModels => Entity::has_many(super::email::Entity).into(),
            Relation::SshKeysModels => Entity::has_many(super::ssh_key::Entity).into(),
            Relation::TokenKeysModels => Entity::has_many(super::token_key::Entity).into(),
            Relation::StarModels => Entity::has_many(super::star::Entity).into(),
            Relation::FollowerModel => Entity::has_many(super::follower::Entity).into(),
            Relation::TwoFactor => Entity::has_many(super::two_factor::Entity).into(),
            // Relation::TeamsModel => Entity::has_many(teamsus::Entity).into(),
            Relation::RepoModel => Entity::has_many(repos::Entity).into(),
        }
    }
}

impl ActiveModel {
    pub fn new_users(username : String, password : String, main_email : String) -> ActiveModel {
        let uid = Uuid::new_v4();
        let password = password.digest();
        ActiveModel {
            uid : Set(uid),
            name : Set(username.clone()),
            username : Set(username),
            bio : Set(None),
            pronouns : Set(None),
            company : Set(None),
            location : Set(None),
            localtime : Set(None),
            i18n : Set(None),
            website : Set(vec![]),
            orcid : Set(None),
            social : Set(vec![]),
            theme : Set("system".to_string()),
            pinned : Set(vec![]),
            followers : Set(0),
            following : Set(0),
            repository : Set(0),
            stars : Set(0),
            watching : Set(0),
            package : Set(0),
            release : Set(0),
            password : Set(password),
            main_email : Set(main_email),
            visible_email : Set(false),
            created : Set(chrono::Local::now().timestamp()),
            updated : Set(chrono::Local::now().timestamp()),

            mentioned : Set(true),
            pro : Set(false),
            avatar_url : Set(None),
            hasused : Set(chrono::Local::now().timestamp()),
            allow_use : Set(false),
            member : Set(vec![]),
        }
    }

    pub fn update_optional(&self, option : UpdateOption) -> ActiveModel {
        let mut model = self.clone();
        if let Some(name) = &option.name {
            model.name = Set(name.clone());
        }
        if let Some(bio) = &option.bio {
            model.bio = Set(Some(bio.clone()));
        }
        if let Some(pronouns) = &option.pronouns {
            model.pronouns = Set(Some(pronouns.clone()));
        }
        if let Some(company) = &option.company {
            model.company = Set(Some(company.clone()));
        }
        if let Some(location) = &option.location {
            model.location = Set(Some(location.clone()));
        }
        if let Some(localtime) = &option.localtime {
            model.localtime = Set(Some(localtime.clone()));
        }
        if let Some(i18n) = &option.i18n {
            model.i18n = Set(Some(i18n.clone()));
        }
        if let Some(website) = &option.website {
            model.website = Set(website.clone());
        }
        if let Some(orcid) = &option.orcid {
            model.orcid = Set(Some(orcid.clone()));
        }
        if let Some(theme) = &option.theme {
            model.theme = Set(theme.clone());
        }
        if let Some(orcid) = &option.orcid {
            model.orcid = Set(Some(orcid.clone()));
        }
        if let Some(orcid) = &option.orcid {
            model.orcid = Set(Some(orcid.clone()));
        }
        if let Some(social) = &option.social {
            model.social = Set(social.clone());
        }
        if let Some(pinned) = &option.pinned {
            model.pinned = Set(pinned.clone());
        }
        model.updated = Set(chrono::Local::now().timestamp());
        model
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UpdateOption {
    pub name : Option<String>,
    pub bio : Option<String>,
    pub pronouns : Option<String>,
    pub company : Option<String>,
    pub location : Option<String>,
    pub localtime : Option<String>,
    pub i18n : Option<String>,
    pub website : Option<Vec<String>>,
    pub orcid : Option<String>,
    pub social : Option<Vec<String>>,
    pub theme : Option<String>,
    pub pinned : Option<Vec<Uuid>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UsersOption {
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
}

impl From<Model> for UsersOption {
    fn from(model : Model) -> Self {
        Self {
            uid : model.uid,
            name : model.name,
            username : model.username,
            bio : model.bio,
            pronouns : model.pronouns,
            company : model.company,
            location : model.location,
            localtime : model.localtime,
            i18n : model.i18n,
            website : model.website,
            orcid : model.orcid,
            social : model.social,
            theme : model.theme,
            pinned : model.pinned,
            followers : model.followers,
            following : model.following,
            repository : model.repository,
            stars : model.stars,
            watching : model.watching,
            package : model.package,
            release : model.release,
            mentioned : model.mentioned,
            main_email : model.main_email,
            visible_email : model.visible_email,
            pro : model.pro,
            avatar_url : model.avatar_url,
            created : model.created,
            updated : model.updated,
            hasused : model.hasused,
        }
    }
}

impl Model {
    pub fn verify_password(&self, password : &str) -> bool {
        self.password == password.digest()
    }
}

#[derive(DeriveIden)]
pub enum UserMigration {
    #[sea_orm(iden = "users")]
    Table,
    #[sea_orm(iden = "uid")]
    Uid,
    #[sea_orm(iden = "name")]
    Name,
    #[sea_orm(iden = "username")]
    Username,
    #[sea_orm(iden = "bio")]
    Bio,
    #[sea_orm(iden = "pronouns")]
    Pronouns,
    #[sea_orm(iden = "company")]
    Company,
    #[sea_orm(iden = "location")]
    Location,
    #[sea_orm(iden = "localtime")]
    Localtime,
    #[sea_orm(iden = "i18n")]
    I18n,
    #[sea_orm(iden = "website")]
    Website,
    #[sea_orm(iden = "orcid")]
    Orcid,
    #[sea_orm(iden = "social")]
    Social,
    #[sea_orm(iden = "theme")]
    Theme,
    #[sea_orm(iden = "pinned")]
    Pinned,
    #[sea_orm(iden = "followers")]
    Followers,
    #[sea_orm(iden = "following")]
    Following,
    #[sea_orm(iden = "repository")]
    Repository,
    #[sea_orm(iden = "stars")]
    Stars,
    #[sea_orm(iden = "watching")]
    Watching,
    #[sea_orm(iden = "package")]
    Package,
    #[sea_orm(iden = "release")]
    Release,
    #[sea_orm(iden = "mentioned")]
    Mentioned,
    #[sea_orm(iden = "main_email")]
    MainEmail,
    #[sea_orm(iden = "visible_email")]
    VisibleEmail,
    #[sea_orm(iden = "pro")]
    Pro,
    #[sea_orm(iden = "password")]
    Password,
    #[sea_orm(iden = "avatar_url")]
    AvatarUrl,
    #[sea_orm(iden = "created")]
    Created,
    #[sea_orm(iden = "updated")]
    Updated,
    #[sea_orm(iden = "hasused")]
    Hasused,
    #[sea_orm(iden = "allow_use")]
    AllowUse,
    #[sea_orm(iden = "member")]
    Member,
}

impl UserMigration {
    pub fn create() -> TableCreateStatement {
        sea_orm_migration::prelude::Table::create()
            .if_not_exists()
            .table(UserMigration::Table)
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Uid)
                    .uuid()
                    .not_null()
                    .primary_key()
                    .unique_key(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Name)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Username)
                    .string()
                    .not_null()
                    .unique_key(),
            )
            .col(sea_orm_migration::prelude::ColumnDef::new(UserMigration::Bio).string())
            .col(sea_orm_migration::prelude::ColumnDef::new(UserMigration::Pronouns).string())
            .col(sea_orm_migration::prelude::ColumnDef::new(UserMigration::Company).string())
            .col(sea_orm_migration::prelude::ColumnDef::new(UserMigration::Location).string())
            .col(sea_orm_migration::prelude::ColumnDef::new(UserMigration::Localtime).string())
            .col(sea_orm_migration::prelude::ColumnDef::new(UserMigration::I18n).string())
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Website)
                    .array(ColumnType::Text)
                    .not_null(),
            )
            .col(sea_orm_migration::prelude::ColumnDef::new(UserMigration::Orcid).string())
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Social)
                    .array(ColumnType::Text)
                    .not_null(),
            )
            .col(sea_orm_migration::prelude::ColumnDef::new(UserMigration::Theme).string())
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Pinned)
                    .array(ColumnType::Uuid)
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Followers)
                    .integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Following)
                    .integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Repository)
                    .integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Stars)
                    .integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Watching)
                    .integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Package)
                    .integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Release)
                    .integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Mentioned)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::MainEmail)
                    .string()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::VisibleEmail)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Pro)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Password)
                    .string()
                    .not_null(),
            )
            .col(sea_orm_migration::prelude::ColumnDef::new(UserMigration::AvatarUrl).string())
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Created)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Updated)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Hasused)
                    .big_integer()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::AllowUse)
                    .boolean()
                    .not_null(),
            )
            .col(
                sea_orm_migration::prelude::ColumnDef::new(UserMigration::Member)
                    .array(ColumnType::Uuid)
                    .not_null(),
            )
            .take()
    }
    pub fn drop() -> sea_orm_migration::prelude::TableDropStatement {
        sea_orm_migration::prelude::Table::drop()
            .table(Self::Table)
            .if_exists()
            .take()
    }
}
