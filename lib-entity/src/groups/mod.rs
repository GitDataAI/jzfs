use sea_orm::prelude::async_trait::async_trait;
use sea_orm::DbErr;
use sea_orm_migration::{MigrationName, SchemaManager};

pub mod groups;




pub struct GroupsMigrator;

impl MigrationName for GroupsMigrator {
    fn name(&self) -> &str {
        "GroupsMigrator"
    }
}

#[async_trait]
impl sea_orm_migration::MigrationTrait for GroupsMigrator {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(groups::GroupsMigration::create())
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::sea_query::PostgresQueryBuilder;
    use super::*;
    
    #[test]
    fn groups_sql() {
        let mut result = Vec::new();
        result.push(
            groups::GroupsMigration::create()
                .to_string( PostgresQueryBuilder {})
        );
        println!("{}", result.join(";\n"));
    }
}