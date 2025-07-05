use crate::migra::stable;
pub use sea_orm_migration::prelude::*;

mod migra;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(stable::Migration)]
    }
}
