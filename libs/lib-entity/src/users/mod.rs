use sea_orm::DbErr;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm_migration::MigrationName;
use sea_orm_migration::SchemaManager;

pub mod email;
pub mod follower;
pub mod ssh_key;
pub mod star;
pub mod token_key;
pub mod two_factor;
pub mod users;
pub mod watchs;

pub struct UsersMigrator;

impl MigrationName for UsersMigrator {
    fn name(&self) -> &str {
        "UsersMigrator"
    }
}

#[async_trait]
impl sea_orm_migration::MigrationTrait for UsersMigrator {
    async fn up(&self, manager : &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(users::UserMigration::create()).await?;
        manager
            .create_table(email::EmailMigration::create())
            .await?;
        manager
            .create_table(follower::FollowerMigration::create())
            .await?;
        manager
            .create_table(ssh_key::SshKeyMigration::create())
            .await?;
        manager.create_table(star::StarMigration::create()).await?;
        manager
            .create_table(token_key::TokenKeyMigration::create())
            .await?;
        manager
            .create_table(two_factor::TwoFactorMigration::create())
            .await?;
        manager
            .create_table(watchs::WatchUserMigration::create())
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sea_query::PostgresQueryBuilder;

    #[tokio::test]
    async fn users_sql() {
        let mut manager = Vec::new();
        manager.push(users::UserMigration::create().to_string(PostgresQueryBuilder {}));
        manager.push(email::EmailMigration::create().to_string(PostgresQueryBuilder {}));
        manager.push(follower::FollowerMigration::create().to_string(PostgresQueryBuilder {}));
        manager.push(ssh_key::SshKeyMigration::create().to_string(PostgresQueryBuilder {}));
        manager.push(star::StarMigration::create().to_string(PostgresQueryBuilder {}));
        manager.push(token_key::TokenKeyMigration::create().to_string(PostgresQueryBuilder {}));
        manager.push(two_factor::TwoFactorMigration::create().to_string(PostgresQueryBuilder {}));
        manager.push(watchs::WatchUserMigration::create().to_string(PostgresQueryBuilder {}));
        println!("{}", manager.join("\n"));
    }
}
