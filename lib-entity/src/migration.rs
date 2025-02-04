use crate::*;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(groups::GroupsMigrator),
            Box::new(users::UsersMigrator),
            Box::new(teams::TeamsMigrator),
            Box::new(repos::ReposMigrator),
        ]
    }
}


impl Migrator {
    pub async fn run(db: DatabaseConnection) -> Result<(), DbErr> {
        unsafe {
            std::env::set_var("DATABASE_URL", "postgres://a:a@a:5432/postgres");
        }
        sea_orm_migration::cli::run_cli_with_connection(
            Self,
            async|_| { Ok(db) },
        )
            .await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_migrator() {
        let table = Migrator::get_migration_files();
        println!("Migration table name: {:?}", table.iter().map(|x|x.name()).map(|x|x.to_string()).collect::<Vec<String>>());
    }
}