use sea_orm::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "blobtree")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub commit_id: Uuid,
    pub branch: String,
    pub tree: JsonValue,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::repos::Entity",
        from = "Column::RepoId",
        to = "super::repos::Column::Uid",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Repos,
}

impl Related<super::repos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Repos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
