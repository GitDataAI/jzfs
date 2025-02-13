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
pub use self::interface::LoadError;
pub use self::interface::SaveError;
pub use self::interface::SessionStore;
pub use self::interface::UpdateError;
#[cfg(feature = "redis-session")]
pub use self::redis_rs::RedisSessionStore;
#[cfg(feature = "redis-session")]
pub use self::redis_rs::RedisSessionStoreBuilder;
pub use self::session_key::SessionKey;
pub use self::utils::generate_session_key;
