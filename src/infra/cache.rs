use web_app_skeleton::base::cache::BaseCache;
use web_app_skeleton::base::error::CustomError;

pub struct RedisCache {

}

impl BaseCache for RedisCache {
    fn set(&self, key: String, value: String) -> Result<(), CustomError> {
        todo!()
    }

    fn get(&self, key: String) -> Result<String, CustomError> {
        todo!()
    }

    fn forget(&self, key: String) -> Result<(), CustomError> {
        todo!()
    }
}