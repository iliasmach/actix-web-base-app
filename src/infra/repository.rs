use web_app_skeleton::base::entity::{BaseEntity, EntityError};
use crate::domain::model::user::User;
use crate::domain::repository::user_repository::UserRepository;
use crate::infra::db;
use crate::infra::db::DbConnection;

pub struct UserRepositoryDB {
    conn: DbConnection,
}

impl UserRepositoryDB {
    pub(crate) fn new() -> Self {
        let conn = db::connection().expect("Cannot get database connection");
        Self { conn }
    }
}


impl UserRepository for UserRepositoryDB
{
    fn find(&self, id: i64) -> Result<User, EntityError> {
        Ok(User::new())
    }
}
