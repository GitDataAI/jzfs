use serde::Deserialize;

pub struct TeamCreate{
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct TeamInvite{
    pub email: String,
}