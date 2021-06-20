use web_app_skeleton::base::entity::EntityError;
use crate::domain::model::user::User;

pub trait UserRepository{
     fn find(&self, id: i64) -> Result<User, EntityError>;
}