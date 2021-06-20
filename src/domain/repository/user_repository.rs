use web_app_skeleton::base::entity::EntityError;
use crate::domain::model::user::User;
use web_app_skeleton::base::error::CustomError;

pub trait UserRepository{
     fn new() -> Self;
     fn find(&self, id: i64) -> Result<User, CustomError>;
}