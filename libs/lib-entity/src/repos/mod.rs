use sea_orm::DbErr;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm_migration::MigrationName;
use sea_orm_migration::SchemaManager;

pub mod blobtree;
pub mod branchs;
pub mod commits;
pub mod labels;
pub mod license;
pub mod repos;

pub struct ReposMigrator;

impl MigrationName for ReposMigrator {
    fn name(&self) -> &str {
        "ReposMigrator"
    }
}

#[async_trait]
impl sea_orm_migration::MigrationTrait for ReposMigrator {
    async fn up(&self, manager : &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(repos::ReposMigration::create())
            .await?;
        manager
            .create_table(branchs::BranchMigration::create())
            .await?;
        manager
            .create_table(commits::CommitMigration::create())
            .await?;
        manager
            .create_table(labels::LabelsMigration::create())
            .await?;
        manager
            .create_table(license::LicenseMigration::create())
            .await?;
        manager
            .create_table(blobtree::BlobTreeMigration::create())
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sea_orm_migration::sea_query::PostgresQueryBuilder;

    use super::*;
    #[tokio::test]
    async fn repos_sql() {
        let mut manager = Vec::new();
        manager.push(repos::ReposMigration::create().to_string(PostgresQueryBuilder {}));
        manager.push(branchs::BranchMigration::create().to_string(PostgresQueryBuilder {}));
        manager.push(commits::CommitMigration::create().to_string(PostgresQueryBuilder {}));
        manager.push(labels::LabelsMigration::create().to_string(PostgresQueryBuilder {}));
        manager.push(license::LicenseMigration::create().to_string(PostgresQueryBuilder {}));
        manager.push(blobtree::BlobTreeMigration::create().to_string(PostgresQueryBuilder {}));
        println!("{}", manager.join("\n"))
    }
}
