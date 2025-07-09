use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "security")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: Uuid,

    pub title: String,
    pub description: Option<String>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub device: Option<String>,
    pub location: Option<String>,

    pub action: String,
    pub actor: String,
    pub actor_uid: Uuid,

    pub user: String,
    pub user_uid: Uuid,
    pub timestamp: chrono::NaiveDateTime,
}

pub struct SecurityOption {
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub device: Option<String>,
    pub location: Option<String>,
}

impl Model {
    /// <h3>User Login</h3><br/>
    /// ```
    /// use cert::models::security::Model;
    /// Model {
    /// uid: Default::default(),
    /// title: Model::USER_LOGIN.to_string(),
    /// description: None,
    /// ip: None,
    /// user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36".to_string()),
    /// device: Some("Windows".to_string()),
    /// location: None,
    /// action: "LOGIN".to_string(),
    /// actor: "$USERNAME".to_string(),
    /// actor_uid: Default::default(),// $USER_UID
    /// user: "$USERNAME".to_string() ,
    /// user_uid: Default::default(), // $USER_UID
    /// timestamp: Default::default() // Utc::now()
    /// }
    /// ```
    pub const USER_LOGIN: &'static str = "user_login";

    /// User logout
    /// ```
    /// use cert::models::security::Model;
    /// Model {
    /// uid: Default::default(),
    /// title: Model::USER_LOGOUT.to_string(),
    /// description: None,
    /// ip: None,
    /// user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36".to_string()),
    /// device: Some("Windows".to_string()),
    /// location: None,
    /// action: "USER_LOGOUT".to_string(),
    /// actor: "$USERNAME".to_string(),
    /// actor_uid: Default::default(),// $USER_UID
    /// user: "$USERNAME".to_string() ,
    /// user_uid: Default::default(), // $USER_UID
    /// timestamp: Default::default() // Utc::now()
    /// }
    /// ```
    pub const USER_LOGOUT: &'static str = "user_logout";

    /// User reset password
    /// ```
    /// use cert::models::security::Model;
    /// Model {
    /// uid: Default::default(),
    /// title: Model::USER_RE_PASSWORD.to_string(),
    /// description: None,
    /// ip: None,
    /// user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36".to_string()),
    /// device: Some("Windows".to_string()),
    /// location: None,
    /// action: "USER RESET PASSWORD".to_string(),
    /// actor: "$USERNAME".to_string(),
    /// actor_uid: Default::default(),// $USER_UID
    /// user: "$USERNAME".to_string() ,
    /// user_uid: Default::default(), // $USER_UID
    /// timestamp: Default::default() // Utc::now()
    /// }
    /// ```
    pub const USER_RE_PASSWORD: &'static str = "user_re_password";
    /// ...And so on
    pub const SSHKEY_REGISTER: &'static str = "sshkey_register";
    /// ...And so on
    pub const SSHKEY_DELETE: &'static str = "sshkey_delete";
    /// ...And so on
    pub const ACCESS_KEY_REGISTER: &'static str = "access_key_register";
    /// ...And so on
    pub const ACCESS_KEY_DELETE: &'static str = "access_key_delete";
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
