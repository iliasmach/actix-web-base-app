use crate::infra::repository::UserRepositoryDB;
use crate::infra::cache::RedisCache;
use crate::domain::service::user_service::UserServiceImpl;
use actix::{Addr, SyncArbiter, Context, AsyncContext, Actor};

pub struct Application {
    pub user_service: Addr<UserServiceImpl<UserRepositoryDB>>,
    pub cache: RedisCache,
}

impl Application {
    pub fn new() -> Self {

        let addr = UserServiceImpl::new(
            SyncArbiter::start(4, || {
                UserRepositoryDB::new()
            })
        ).start();
        Self {
            user_service:addr,
            cache: RedisCache {},
        }
    }
}
