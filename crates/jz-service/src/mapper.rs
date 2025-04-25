use jz_model_sqlx::users::UserMapper;
use crate::app::AppService;

impl AppService {
    pub fn user_mapper(&self) -> UserMapper {
        UserMapper{
            db: self.read.clone(),
        }
    }
    pub fn token_mapper(&self) -> jz_model_sqlx::token::TokenMapper {
        jz_model_sqlx::token::TokenMapper{
            db: self.read.clone(),
        }
    }
    pub fn ssh_key_mapper(&self) -> jz_model_sqlx::ssh_key::SshKeyMapper {
        jz_model_sqlx::ssh_key::SshKeyMapper{
            db: self.read.clone()
        }
    }
    pub fn secrets_mapper(&self) -> jz_model_sqlx::secrets::SecretsMapper {
        jz_model_sqlx::secrets::SecretsMapper{
            db: self.read.clone()
        }
    }
}