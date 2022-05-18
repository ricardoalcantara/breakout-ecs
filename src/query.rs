use crate::{component::ComponentVec, world::World};
use std::{
    any::Any,
    cell::{RefCell, RefMut},
    rc::Rc,
};

pub trait Query {
    type Item;
    fn fetch(world: &mut World) -> QueryResult<Self::Item>;
}

pub struct QueryResult<T> {
    pub result: Vec<T>,
}

impl<T> IntoIterator for QueryResult<T> {
    type Item = T;
    type IntoIter = QueryResultIntoIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        QueryResultIntoIterator {
            result: self.result,
            index: 0,
        }
    }
}

pub struct QueryResultIntoIterator<T> {
    result: Vec<T>,
    index: usize,
}

impl<T> Iterator for QueryResultIntoIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.result.len() > 0 {
            Some(self.result.remove(0))
        } else {
            None
        }
    }
}

impl<A: 'static> Query for (A,) {
    type Item = (usize, Rc<RefCell<A>>);
    fn fetch(world: &mut World) -> QueryResult<Self::Item> {
        let a: Vec<(usize, Rc<RefCell<A>>)> = world
            .borrow_component_vec::<A>()
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(i, f)| Some((i, f.as_ref()?.clone())))
            .collect();

        QueryResult { result: a }
    }
}

impl<A: 'static, B: 'static> Query for (A, B) {
    type Item = (usize, (Rc<RefCell<A>>, Rc<RefCell<B>>));
    fn fetch(world: &mut World) -> QueryResult<Self::Item> {
        let a = world.borrow_component_vec::<A>().unwrap();
        let b = world.borrow_component_vec::<B>().unwrap();

        let r = a
            .iter()
            .zip(b.iter())
            .enumerate()
            .filter_map(|(i, (f1, f2))| Some((i, (f1.as_ref()?.clone(), f2.as_ref()?.clone()))))
            .collect();

        QueryResult { result: r }
    }
}

impl<A: 'static, B: 'static, C: 'static> Query for (A, B, C) {
    type Item = (usize, (Rc<RefCell<A>>, Rc<RefCell<B>>, Rc<RefCell<C>>));
    fn fetch(world: &mut World) -> QueryResult<Self::Item> {
        let a = world.borrow_component_vec::<A>().unwrap();
        let b = world.borrow_component_vec::<B>().unwrap();
        let c = world.borrow_component_vec::<C>().unwrap();

        let r = a
            .iter()
            .zip(b.iter())
            .zip(c.iter())
            .enumerate()
            .filter_map(|(i, ((f1, f2), f3))| {
                Some((
                    i,
                    (
                        f1.as_ref()?.clone(),
                        f2.as_ref()?.clone(),
                        f3.as_ref()?.clone(),
                    ),
                ))
            })
            .collect();

        QueryResult { result: r }
    }
}

impl<A: 'static, B: 'static, C: 'static, D: 'static> Query for (A, B, C, D) {
    type Item = (
        usize,
        (
            Rc<RefCell<A>>,
            Rc<RefCell<B>>,
            Rc<RefCell<C>>,
            Rc<RefCell<D>>,
        ),
    );
    fn fetch(world: &mut World) -> QueryResult<Self::Item> {
        let a = world.borrow_component_vec::<A>().unwrap();
        let b = world.borrow_component_vec::<B>().unwrap();
        let c = world.borrow_component_vec::<C>().unwrap();
        let d = world.borrow_component_vec::<D>().unwrap();

        let r = a
            .iter()
            .zip(b.iter())
            .zip(c.iter())
            .zip(d.iter())
            .enumerate()
            .filter_map(|(i, (((f1, f2), f3), f4))| {
                Some((
                    i,
                    (
                        f1.as_ref()?.clone(),
                        f2.as_ref()?.clone(),
                        f3.as_ref()?.clone(),
                        f4.as_ref()?.clone(),
                    ),
                ))
            })
            .collect();

        QueryResult { result: r }
    }
}
