use std::any::Any;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct ContainerIOC {
    inner: Arc<Mutex<Vec<Box<dyn Send + Sync + 'static + Any>>>>,
}

impl ContainerIOC {
    pub async fn init() -> ContainerIOC {
        Self {
            inner: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn take<T: 'static>(&self) -> Option<Arc<T>> {
        let inner = self.inner.lock().unwrap();
        for item in inner.iter() {
            if let Some(arc_item) = item.downcast_ref::<Arc<T>>() {
                return Some(arc_item.clone());
            }
        }
        drop(inner);
        None
    }

    pub fn inject<T: 'static + Send + Sync>(&self, item: T) {
        let mut inner = self.inner.lock().unwrap();
        inner.push(Box::new(Arc::new(item)));
        drop(inner);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ioc() {
        let ioc = ContainerIOC::init().await;
        ioc.inject("str".to_string());
        ioc.inject(2_i32);
        assert_eq!(
            ioc.take::<String>().await,
            Some(Arc::new("str".to_string()))
        );
        assert_eq!(ioc.take::<i32>().await, Some(Arc::new(2_i32)));
    }
}
