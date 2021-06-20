use crate::base::entity::{BaseEntity, EntityError};

pub trait BaseRepository<ID, E: BaseEntity<ID>>
    where
        ID: PartialEq
{
    fn find(&self, id: ID) -> Result<E, EntityError>;

    fn find_all(&self) -> Result<Vec<E>, EntityError>;
    fn delete(&self, id: ID) -> Result<i32, EntityError>;
}