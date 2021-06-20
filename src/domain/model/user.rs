use web_app_skeleton::base::entity::BaseEntity;

#[derive(Debug)]
pub struct User {
    id: i64
}

impl BaseEntity<i64> for User {
    fn new() -> Self {
        User {
            id: 0
        }
    }

    fn id(&self) -> i64 {
        self.id
    }
}