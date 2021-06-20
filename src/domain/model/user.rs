use web_app_skeleton::base::entity::BaseEntity;
use std::any::Any;
use std::ops::Deref;

#[derive(Debug)]
pub struct User {
    id: i64
}

impl BaseEntity<i64> for User {
    fn new() -> Self where Self: Sized {
        User { id: 0 }
    }


    fn id(&self) -> i64 {
        self.id
    }
}

