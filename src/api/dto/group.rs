use serde::Deserialize;


#[derive(Deserialize)]
pub struct GroupCreate{
    pub name: String,
    pub contact: String,
    pub description: String,
}