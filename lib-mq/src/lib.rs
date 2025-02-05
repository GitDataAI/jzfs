#![feature(lock_value_accessors)]

pub mod server;
pub mod client;

pub const CHANNEL: &str = "app";
pub const EMAIL_TOPIC: &str = "email";