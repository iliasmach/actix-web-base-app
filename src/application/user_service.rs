
use actix::{Actor, Handler, ResponseFuture, Context};
use crate::domain::service::user_service::UserServiceImpl;
use crate::infra::repository::UserRepositoryDB;
use crate::application::messages::user::UserExist;

use web_app_skeleton::base::messages::Find;
use std::time::Duration;
use crate::domain::model::user::User;


impl Actor for UserServiceImpl<UserRepositoryDB> {
    type Context = Context<UserServiceImpl<UserRepositoryDB>>;
}

impl Handler<UserExist> for UserServiceImpl<UserRepositoryDB> {
    type Result = ResponseFuture<Result<User, ()>>;

    fn handle(&mut self, msg: UserExist, _ctx: & mut Context<Self>) -> Self::Result {
let repo = self.repo.clone();
        Box::pin(async move{
            Ok(repo.send(Find::new(msg.id)).timeout(
                Duration::from_secs(2)
            ).await.unwrap().unwrap())
        })
    }
}