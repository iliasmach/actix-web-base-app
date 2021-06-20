#[derive(Debug)]
pub enum EntityError {
    EntityNotFound
}

pub trait BaseEntity<ID>
    where
        ID: PartialEq, Self: Send{
    fn new() -> Self where Self : Sized;
    fn id(&self) -> ID;

}