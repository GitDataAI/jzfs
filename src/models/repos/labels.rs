use sea_orm::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "labels")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub url: String,
    pub name: String,
    pub color: String,
    #[sea_orm(column_name = "description")]
    pub description: Option<String>,
    pub created: i64,
}
impl ActiveModelBehavior for ActiveModel {}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    ModelHasRepoLabels,
}
impl Related<super::repos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ModelHasRepoLabels.def()
    }
}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::ModelHasRepoLabels => Entity::belongs_to(super::repos::Entity)
                .from(Column::RepoId)
                .to(super::repos::Column::Uid)
                .into(),
        }
    }
}
