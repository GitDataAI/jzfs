//! Pluggable storage backends for session state.

mod cookie;
pub(crate) mod interface;
mod session_key;
mod utils;

pub use self::{
    interface::{LoadError, SaveError, SessionStore, UpdateError},
    session_key::SessionKey,
    utils::generate_session_key,
};
