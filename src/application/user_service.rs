use crate::domain::repository::user_repository::UserRepository;
use crate::base::service::BaseService;

use std::ops::Deref;
use std::any::Any;

pub trait UserServiceApp<T: UserRepository> {
    fn user_service(&mut self, service: UserServiceImpl<T>);
    fn get_user_service(&self) -> &UserServiceImpl<T>;
}


pub struct UserServiceImpl<T: UserRepository>
{
    repo: T,
}

impl<T:UserRepository> Clone for UserServiceImpl<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T: UserRepository> UserServiceImpl<T>
{
    pub fn new(repo: T) -> Self {
        Self { repo }
    }

    pub fn has_user(&self, id: i64) -> bool {
        match self.repo.find(id) {
            Ok(_) => true,
            Err(_e) => false
        }
    }
}

impl<T: UserRepository> BaseService for UserServiceImpl<T> {}
