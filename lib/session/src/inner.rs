use std::cell::RefCell;
use std::rc::Rc;
use dashmap::DashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use time::Duration;

#[derive(Debug,Eq, PartialEq,Clone)]
pub enum SessionStatus {
    Unchanged,
    Change,
    Renewed,
    Purge,
}



#[derive(Debug)]
pub struct SessionInner {
    pub status: SessionStatus,
    pub map: DashMap<String, String>,
    pub expires: Option<Duration>,
}

#[derive(Clone)]
pub struct Session(pub Rc<RefCell<SessionInner>>);


impl Default for SessionInner {
    fn default() -> Self {
        Self {
            status: SessionStatus::Unchanged,
            map: DashMap::new(),
            expires: None,
        }
    }
}

impl Session {
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> anyhow::Result<T> {
        let inner = self.0.borrow();
        let value = inner.map.get(key).ok_or(anyhow::anyhow!("key not found"))?.clone();
        Ok(serde_json::from_str(&value)?)
    }
    pub fn set<T: Serialize>(&self, key: &str, value: T) {
        let mut inner = self.0.borrow_mut();
        inner.map.insert(key.to_string(), serde_json::to_string(&value).unwrap());
        inner.status = SessionStatus::Change;
    }
    pub fn remove(&self, key: &str) {
        let mut inner = self.0.borrow_mut();
        inner.map.remove(key);
        inner.status = SessionStatus::Change;
    }
    pub fn is_purge(&self) -> bool {
        self.0.borrow().status.clone() == SessionStatus::Purge
    }
    pub fn is_change(&self) -> bool {
        self.0.borrow().status.clone() == SessionStatus::Change
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    pub fn test_session(){
        let inner = SessionInner::default();
        let session = Session(Rc::new(RefCell::new(inner)));
        // step 1
        {
            session.set("username", "admin");
            session.set("password", "123456");
        }
        // step 2
        {
            let username = session.get::<String>("username");
            let password = session.get::<String>("password");
            assert!(username.is_ok());
            assert!(password.is_ok());
            assert_eq!(username.unwrap(), "admin");
            assert_eq!(password.unwrap(), "123456");
        }
        // step 3
        {
            session.remove("username");
            session.remove("password");
        }
        // step 4
        {
            let username = session.get::<String>("username");
            let password = session.get::<String>("password");
            assert!(username.is_err());
            assert!(password.is_err());
        }
    }
}