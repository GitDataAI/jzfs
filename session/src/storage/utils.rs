use rand::distr::{Alphanumeric, SampleString as _};

use crate::storage::SessionKey;

pub fn generate_session_key() -> SessionKey {
    Alphanumeric
        .sample_string(&mut rand::rng(), 64)
        .try_into()
        .expect("generated string should be within size range for a session key")
}
