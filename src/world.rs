use std::cell::{RefCell, RefMut};

use super::{
    component::{ComponentBundle, ComponentVec},
    query::{QueryIterator, QueryParameters},
};

pub struct World {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }
    pub fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for component_vec in self.component_vecs.iter_mut() {
            // The `downcast_mut` type here is changed to `RefCell<Vec<Option<ComponentType>>`
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                // add a `get_mut` here. Once again `get_mut` bypasses
                // `RefCell`'s runtime checks if accessing through a `&mut` reference.
                component_vec.get_mut()[entity] = Some(component);
                return;
            }
        }

        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);

        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(component);

        // Here we create a `RefCell` before inserting into `component_vecs`
        self.component_vecs
            .push(Box::new(RefCell::new(new_component_vec)));
    }

    pub fn borrow_component_vec<ComponentType: 'static>(
        &self,
    ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                // Here we use `borrow_mut`.
                // If this `RefCell` is already borrowed from this will panic.
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }

    //https://stackoverflow.com/a/56700760/8378479
    pub fn spawn(&mut self, bundle: impl ComponentBundle) -> usize {
        let entity = self.new_entity();
        bundle.spawn_in_world(self, entity);

        entity
    }

    pub fn get(&self, _entity: usize) {}
    pub fn get_mut(&self, _entity: usize) {}

    pub fn query<T: QueryParameters>(&self) -> QueryIterator<T> {
        todo!()
    }

    pub fn query_mut<T: QueryParameters>(&self) -> QueryIterator<T> {
        todo!()
    }
}
