use derive_more::derive::{Display, From};

#[derive(Debug, PartialEq, Eq)]
pub struct SessionKey(pub(crate) String);

impl TryFrom<String> for SessionKey {
    type Error = InvalidSessionKeyError;

    fn try_from(val: String) -> Result<Self, Self::Error> {
        if val.len() > 4064 {
            return Err(anyhow::anyhow!(
                "The session key is bigger than 4064 bytes, the upper limit on cookie content."
            )
            .into());
        }

        Ok(SessionKey(val))
    }
}

impl AsRef<str> for SessionKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<SessionKey> for String {
    fn from(key: SessionKey) -> Self {
        key.0
    }
}

#[derive(Debug, Display, From)]
#[display("The provided string is not a valid session key")]
pub struct InvalidSessionKeyError(anyhow::Error);

impl std::error::Error for InvalidSessionKeyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.0.as_ref())
    }
}
