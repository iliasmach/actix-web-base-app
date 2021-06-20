use actix::Message;
use crate::domain::model::user::User;

pub struct UserExist {
    pub id: i64
}

impl Message for UserExist { type Result = Result<User, ()>; }