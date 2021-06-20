use async_trait::async_trait;
use crate::base::error::CustomError;

#[async_trait]
pub trait BaseCache {
    fn set(&self, key: String, value: String) -> Result<(), CustomError>;
    fn get(&self, key: String) -> Result<String, CustomError>;
    fn forget(&self, key: String) -> Result<(), CustomError>;
}