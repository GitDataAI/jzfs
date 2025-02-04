//! Pluggable storage backends for session state.

#[cfg(feature = "cookie-session")]
pub mod cookie;
pub mod interface;
#[cfg(feature = "redis-session")]
pub mod redis_rs;
pub mod session_key;
pub mod utils;

#[cfg(feature = "cookie-session")]
pub use self::cookie::CookieSessionStore;
#[cfg(feature = "redis-session")]
pub use self::redis_rs::{RedisSessionStore, RedisSessionStoreBuilder};
pub use self::{
    interface::{LoadError, SaveError, SessionStore, UpdateError},
    session_key::SessionKey,
    utils::generate_session_key,
};
