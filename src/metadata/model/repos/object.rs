use sea_orm::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize,Serialize,Debug,Clone,DeriveEntityModel)]
#[sea_orm(table_name = "object")]
pub struct Model{
    #[sea_orm(primary_key)]
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub branch_id: Uuid,
    pub object: String,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct ObjectModel{
    pub uid: Uuid,
    pub repo_id: Uuid,
    pub branch_id: Uuid,
    pub object: Vec<Object>
}



#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[derive(Deserialize,Serialize,Debug,Clone)]
#[derive(PartialEq)]
pub struct Object{
    pub name: String,
    pub path: String,
    pub hash: String,
    pub commit: String,
    pub is_dir: bool,
    pub size: u64,
    pub children: Vec<Object>,
}

impl ActiveModelBehavior for ActiveModel {}


impl Model {
    pub fn to_object_model(&self) -> ObjectModel{
        ObjectModel{
            uid: self.uid,
            repo_id: self.repo_id,
            branch_id: self.branch_id,
            object: serde_json::from_value(json!(self.object)).unwrap()
        }
    }
    pub fn from_object_model(object: &ObjectModel) -> Model{
        Model{
            uid: object.uid,
            repo_id: object.repo_id,
            branch_id: object.branch_id,
            object: serde_json::to_string(&object.object).unwrap()
        }
    }
}

impl ObjectModel {
    pub fn to_model(&self) -> Model {
        Model::from_object_model(self)
    }
}
