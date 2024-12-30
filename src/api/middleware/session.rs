use std::sync::Arc;
use crate::error::{JZError, JZResult};
use crate::models::users::users::Model as UserModel;
use actix_session::Session;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
const SESSION_KEY: &str = "SESSIONID";

pub use Model as SessionModel;
use crate::server::MetaData;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Model {
    pub uid: Uuid,
    pub name: String,
    pub username: String,
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub localtime: Option<String>,
    pub i18n: Option<String>,
    pub website: Vec<String>,
    pub orcid: Option<String>,
    pub social: Vec<String>,
    pub theme: String,
    pub pinned: Vec<Uuid>,
    pub mentioned: bool,
    pub main_email: String,
    pub visible_email: bool,
    pub pro: bool,
    pub avatar_url: Option<String>,
}

impl From<&UserModel> for SessionModel {
    fn from(value: &UserModel) -> Self {
        Self {
            uid: value.uid,
            name: value.name.clone(),
            username: value.username.clone(),
            bio: value.bio.clone(),
            pronouns: value.pronouns.clone(),
            company: value.company.clone(),
            location: value.location.clone(),
            localtime: value.localtime.clone(),
            i18n: value.i18n.clone(),
            website: value.website.clone(),
            orcid: value.orcid.clone(),
            social: value.social.clone(),
            theme: value.theme.clone(),
            pinned: value.pinned.clone(),
            mentioned: value.mentioned,
            main_email: value.main_email.clone(),
            visible_email: value.visible_email,
            pro: value.pro,
            avatar_url: value.avatar_url.clone(),
        }
    }
}
impl SessionModel {
    pub async fn authenticate(session: Session) -> JZResult<SessionModel> {
        match session.get::<SessionModel>(SESSION_KEY) {
            Ok(Some(value)) => Ok(value),
            Ok(None) => Err(JZError::Other(anyhow!("Session not found"))),
            Err(err) => Err(JZError::Other(anyhow!(err))),
        }
    }
    pub async fn insert(&self, session: Session) {
        session.insert(SESSION_KEY, self).ok();
    }
    pub async fn sync(&self, session: Session, meta_data: Arc<MetaData>) -> anyhow::Result<()>{
        let uid = self.uid;
        let model = match meta_data.users_info_uid(uid).await {
            Ok(model) => model,
            Err(err) => return Err(anyhow!(err)),
        };
        let model = SessionModel::from(&model);
        session.insert(SESSION_KEY, model).ok();
        Ok(())
    }
}
