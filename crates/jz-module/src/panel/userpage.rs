// use serde_json::Value;
// use uuid::Uuid;
// use crate::AppModule;
//
// impl AppModule {
//     pub async fn user_page(&self, ops_uid: Uuid, username: String) -> anyhow::Result<serde_json::Value> {
//         let value = Value::Null;
//         let user = self.user_info_by_username(username).await?;
//         let repo = self.repo_info_by_owner_uid(user.uid).await?;
//         Ok(value)
//     }
// }
