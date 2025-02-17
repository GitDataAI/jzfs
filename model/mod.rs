pub mod users;
pub mod repository;
pub mod origin;
pub mod issues;

pub const CREATE_TABLE:&str = include_str!("sql.sql");