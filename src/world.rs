use super::component::{ComponentBundle, ComponentVec};
use crate::query::{Query, QueryResult};
use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

type RcRefCellComponent<T> = Option<Rc<RefCell<T>>>;
type ECSVec<T> = Vec<RcRefCellComponent<T>>;
type RcECSVec<T> = RefCell<ECSVec<T>>;

pub struct World {
    entities_count: usize,
    component_vecs: HashMap<TypeId, Box<dyn ComponentVec>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: HashMap::new(),
        }
    }

    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for (k, component_vec) in &mut self.component_vecs {
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
        let type_id = TypeId::of::<ComponentType>();
        let rc_component = Some(Rc::new(RefCell::new(component)));

        if let Some(mut component_vec) = self.component_vecs.get_mut(&type_id) {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RcECSVec<ComponentType>>()
            {
                component_vec.get_mut()[entity] = rc_component;
                return;
            }
        }

        let mut new_component_vec: ECSVec<ComponentType> = Vec::with_capacity(self.entities_count);

        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = rc_component;

        // // Here we create a `RefCell` before inserting into `component_vecs`
        self.component_vecs
            .insert(type_id, Box::new(RefCell::new(new_component_vec)));
    }

    pub fn borrow_component_vec<ComponentType: 'static>(
        &self,
    ) -> Option<Ref<ECSVec<ComponentType>>> {
        if let Some(mut component_vec) = self.component_vecs.get(&TypeId::of::<ComponentType>()) {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RcECSVec<ComponentType>>()
            {
                return Some(component_vec.borrow());
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

    pub fn query<T: Query>(&mut self) -> QueryResult<T::Item> {
        T::fetch(self)
    }
}
