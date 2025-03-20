use serde::Serialize;
use crate::AppModule;


#[derive(Serialize)]
pub struct CheckName {
    user: bool,
    org: bool
}


impl AppModule {
    pub async fn check_name(&self, name: String) -> anyhow::Result<CheckName> {
        let mut check = CheckName {
            user: false,
            org: false,
        };
        if let Ok(_) = self.user_info_by_username(name.clone()).await {
            check.user = true;
            return Ok(check);
        }
        if let Ok(_) = self.org_by_name(name.clone()).await {
            check.org = true;
            return Ok(check);
        }
        Ok(check)
    }
}