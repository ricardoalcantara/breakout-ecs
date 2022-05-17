use crate::world::{self, World};

pub trait Query {
    type Item<'a>;
    fn new<'a>(world: &'a World) -> QueryBorrow<Self::Item<'a>>;
}

impl<A: 'static> Query for (&A,) {
    type Item<'a> = (&'a A,);
    fn new<'a>(world: &'a World) -> QueryBorrow<Self::Item<'a>> {
        let mut a = world.borrow_component_vec::<A>().unwrap();

        todo!();
    }
}

pub struct QueryBorrow<T: Query> {
    result: T,
}

trait StreamingIterator {
    type Item<'a>
    where
        Self: 'a;
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}
