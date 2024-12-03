use serde::Deserialize;


#[allow(unused)]
#[derive(Deserialize)]
pub struct ListOption<T>{
    pub offset: i64,
    pub limit: i64,
    pub order: String,
    pub filter: T,
}