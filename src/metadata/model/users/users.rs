use sea_orm::*;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;


#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub name: String,
    pub username: String,

    pub phone: Option<String>,
    pub status: i32,

    pub sex: Option<String>,
    pub website: Vec<String>,
    pub company: String,
    pub description: String,

    pub localtime: String,
    pub timezone: String,

    pub theme: String,

    pub pro: bool,

    pub passwd: String,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub lastlogin: OffsetDateTime,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}


impl ActiveModelBehavior for ActiveModel {}