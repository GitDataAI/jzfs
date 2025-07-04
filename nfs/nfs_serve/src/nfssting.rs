use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Default, Clone)]
pub struct nfsstring(pub Vec<u8>);
impl nfsstring {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl From<Vec<u8>> for nfsstring {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}
impl From<&[u8]> for nfsstring {
    fn from(value: &[u8]) -> Self {
        Self(value.into())
    }
}
impl AsRef<[u8]> for nfsstring {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::ops::Deref for nfsstring {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl fmt::Debug for nfsstring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", String::from_utf8_lossy(&self.0))
    }
}
impl fmt::Display for nfsstring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", String::from_utf8_lossy(&self.0))
    }
}