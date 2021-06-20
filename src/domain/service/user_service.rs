use async_trait::async_trait;
use crate::domain::repository::user_repository::UserRepository;
use web_app_skeleton::base::service::BaseService;
use actix::{Addr, Actor, SyncArbiter, Handler, SyncContext};
use web_app_skeleton::base::messages::{Find, Delete};
use crate::domain::model::user::User;
use web_app_skeleton::base::entity::BaseEntity;
use crate::infra::repository::UserRepositoryDB;
use actix::dev::{MessageResponse, ToEnvelope};


#[async_trait]
pub trait UserService {}

pub struct UserServiceImpl<T: UserRepository + Actor + 'static>
{
    pub(crate) repo: Addr<T>,
}


impl<T: UserRepository
+ Actor
+ Handler<Find<i64, User>>
> UserServiceImpl<T>
{
    pub fn new(repo: Addr<T>) -> Self {
        println!("New UserServiceImpl");
        Self { repo }
    }
}

impl<T: UserRepository + Actor> BaseService for UserServiceImpl<T> {}


