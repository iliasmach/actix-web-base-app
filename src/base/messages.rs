use actix::Message;
use crate::base::entity::BaseEntity;
use std::marker::PhantomData;

pub struct Find<ID: PartialEq, A> where A: BaseEntity<ID> {
    id: ID,
    _marker: PhantomData<A>,
}

impl<ID: PartialEq, A: BaseEntity<ID>> Find<ID, A> {
    pub fn new(id: ID) -> Self {
        Find { id, _marker: Default::default() }
    } 
}

impl<ID: PartialEq, A: BaseEntity<ID> + 'static> Message for Find<ID, A> {
    type Result = Result<A, ()>;
}

pub struct Delete<ID: PartialEq, A> where A: BaseEntity<ID> {
    id: ID,
    _marker: PhantomData<A>
}
