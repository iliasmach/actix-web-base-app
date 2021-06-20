use web_app_skeleton::base::entity::{BaseEntity, EntityError};
use crate::domain::model::user::User;
use crate::domain::repository::user_repository::UserRepository;
use crate::infra::db;
use crate::infra::db::DbConnection;
use web_app_skeleton::base::error::CustomError;
use actix::{Actor, ActorContext, SyncContext, Handler};
use web_app_skeleton::base::messages::Find;
use actix::dev::MessageResponse;
use crate::application::messages::user::UserExist;
use crate::domain::service::user_service::UserServiceImpl;

pub struct UserRepositoryDB {
    conn: Option<DbConnection>,
}

impl UserRepositoryDB {
    pub(crate) fn new() -> Self {
        // let conn = db::connection().expect("Cannot get database connection");
        Self { conn:None }
    }
}

impl UserRepository for UserRepositoryDB
{
    fn new() -> Self {
        todo!()
    }

    fn find(&self, id: i64) -> Result<User, CustomError> {
        Ok(User::new())
    }
}

impl Actor for UserRepositoryDB {
    type Context = SyncContext<Self>;
}

impl Handler<Find<i64, User>> for UserRepositoryDB {
    type Result = Result<User, ()>;

    fn handle(&mut self, msg: Find<i64, User>, ctx: &mut SyncContext<Self>) -> Self::Result {
        Ok(User::new())
    }
}