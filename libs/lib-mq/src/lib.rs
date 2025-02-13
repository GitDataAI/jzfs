#![feature(lock_value_accessors)]

pub mod client;
pub mod server;

pub const CHANNEL : &str = "app";
pub const EMAIL_TOPIC : &str = "email";
