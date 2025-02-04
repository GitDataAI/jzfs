pub mod teaminv;
pub mod teamrepo;
pub mod teams;
pub mod teamsus;

use sea_orm::prelude::async_trait::async_trait;
use sea_orm::DbErr;
use sea_orm_migration::{MigrationName, SchemaManager};

pub struct TeamsMigrator;

impl MigrationName for TeamsMigrator {
    fn name(&self) -> &str {
        "TeamsMigrator"
    }
}

#[async_trait]
impl sea_orm_migration::MigrationTrait for TeamsMigrator {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(teaminv::TeamInvMigration::create())
            .await?;
        manager
            .create_table(teamrepo::TeamRepoMigration::create())
            .await?;
        manager
            .create_table(teams::TeamsMigration::create())
            .await?;
        manager
            .create_table(teamsus::TeamUsMigration::create())
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sea_query::PostgresQueryBuilder;

    #[tokio::test]
    async fn teams_sql() {
        let mut result = Vec::new();
        result.push(
            teams::TeamsMigration::create()
                .to_string(PostgresQueryBuilder{})
        );
        result.push(
            teamsus::TeamUsMigration::create()
                .to_string(PostgresQueryBuilder{})
        );
        result.push(
            teaminv::TeamInvMigration::create()
                .to_string(PostgresQueryBuilder{})
        );
        result.push(
            teamrepo::TeamRepoMigration::create()
                .to_string(PostgresQueryBuilder{})
        );
        for i in result {
            println!("{}", i);
        }
    }
}