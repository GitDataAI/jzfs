use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct CheckRepo{
    pub exits: bool,
    pub is_group: bool,
}