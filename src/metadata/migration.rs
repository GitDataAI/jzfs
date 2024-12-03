// use sea_orm::prelude::async_trait::async_trait;
// use sea_orm::Statement;
// use sea_orm_migration::{
//     prelude::*,
// };
// 
// #[derive(DeriveMigrationName,Clone)]
// pub struct Migration;
// 
// #[async_trait]
// impl MigrationTrait for Migration{
//     async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
//         manager.get_connection()
//             .clone()
//             .execute(
//                 Statement::from_string(
//                     manager.get_database_backend(),
//                     r#"
//                         CREATE TABLE IF NOT EXISTS `user` (
//                             `uid` varchar(36) NOT NULL,
//                             `name` varchar(255) NOT NULL,
//                             `email` varchar(255) NOT NULL,
//                             `phone` varchar(255) NULL,
//                             `team` json NOT NULL,
//                             `status` int NOT NULL,
//                             `passwd` varchar(255) NOT NULL,
//                             `created_at` int NOT NULL,
//                        "#
//                 )
//             ).await?;
// 
//         Ok(())
//     }
// }