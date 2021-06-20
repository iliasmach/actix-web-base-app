use async_trait::async_trait;
use crate::base::error::CustomError;
use std::time::Duration;

pub enum CacheError {
    KeyNotFound,
    InternalCacheError
}

#[async_trait]
pub trait BaseCache {
    fn new() -> Result<Box<Self>, CacheError>;
    async fn set(&mut self, key: String, value: String) -> Result<(), CacheError>;
    async fn get(&self, key: String) -> Result<String, CacheError>;
    async fn forget(&mut self, key: String) -> Result<(), CacheError>;
    async fn expire(&mut self, key: String, duration: Duration)-> Result<(), CacheError>;
}