use web_app_skeleton::base::cache::{BaseCache, CacheError};
use web_app_skeleton::base::error::CustomError;
use std::time::Duration;
use async_trait::async_trait;
pub struct RedisCache {

}

#[async_trait]
impl BaseCache for RedisCache {
    fn new() -> Result<Box<Self>, CacheError> {
        todo!()
    }

    async fn set(&mut self, key: String, value: String) -> Result<(), CacheError> {
        todo!()
    }

    async fn get(&self, key: String) -> Result<String, CacheError> {
        todo!()
    }

    async fn forget(&mut self, key: String) -> Result<(), CacheError> {
        todo!()
    }

    async fn expire(&mut self, key: String, duration: Duration) -> Result<(), CacheError> {
        todo!()
    }
}