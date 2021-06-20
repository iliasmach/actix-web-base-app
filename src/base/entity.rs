#[derive(Debug)]
pub enum EntityError {
    EntityNotFound
}

pub trait BaseEntity<ID>
    where
        ID: PartialEq {
    fn new() -> Self;
    fn id(&self) -> ID;

}