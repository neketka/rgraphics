use std::{any::Any, collections::HashMap};

use super::Engine;

trait Component: Clone {
    fn name() -> &'static str;
}

trait System {
    fn name() -> &'static str;
    fn systems_before() -> &'static [&'static str];
    fn systems_after() -> &'static [&'static str];
    fn run(engine: &Engine, dt: f32);
}

pub struct Entity(u64);

impl Entity {
    pub fn new(comps: &[&'static str], registry: WorldRegistry) -> Entity {}
}

struct CompStorageData {
    vec: Box<dyn Any>,
    idToIdx: HashMap<u64, usize>,
    idxToId: Vec<u64>,
}

struct CompStorage {
    new_gen: Box<dyn Fn() -> CompStorageData>,
    insert_gen: Box<dyn Fn(&mut CompStorageData, u64)>,
    delete_gen: Box<dyn Fn(&mut CompStorageData, u64)>,
    get_gen: Box<dyn Fn(&mut CompStorageData, u64) -> &dyn Any>,
    get_by_idx_gen: Box<dyn Fn(&mut CompStorageData, usize) -> &dyn Any>,
    len_gen: Box<dyn Fn(&mut CompStorageData) -> usize>,
}

pub struct WorldRegistry {}

impl WorldRegistry {
    pub fn registerComponent<T: Component>(default: T) {
        let new = || CompStorageData {
            vec: Box::new(Vec::<T>::new()),
            idToIdx: HashMap::new(),
            idxToId: Vec::new(),
        };

        let insert = |data: &mut CompStorageData, id: u64| {
            let v = data.vec.downcast_mut::<Vec<T>>().unwrap();
            let inserted_idx = v.len();

            v.push(default.clone());
            data.idxToId.push(id);
            data.idToIdx.insert(id, inserted_idx);
        };

        let delete = |data: &mut CompStorageData, id: u64| {
            let idx = data.idToIdx.get(&id).unwrap().clone();
            let v = data.vec.downcast_mut::<Vec<T>>().unwrap();

            if v.len() - 1 == idx {
                data.idToIdx.remove(&id);
                data.idxToId.pop();
                v.pop();
                return;
            }

            let loc = v.get_mut(idx).unwrap();
            *loc = v.pop().unwrap();

            let last_id = data.idxToId.pop().unwrap();
            data.idToIdx.remove(&id);
            data.idToIdx.insert(last_id, idx);
        };

        let get = |data: &mut CompStorageData, id: u64| {
            let idx = data.idToIdx.get(&id).unwrap().clone();
            let v = data.vec.downcast_mut::<Vec<T>>().unwrap();
            v.get_mut(idx).unwrap() as &dyn Any
        };

        let get_by_idx = |data: &mut CompStorageData, idx: usize| {
            let v = data.vec.downcast_mut::<Vec<T>>().unwrap();
            v.get_mut(idx).unwrap() as &dyn Any
        };

        let len = |data: &mut CompStorageData| data.vec.downcast_mut::<Vec<T>>().unwrap().len();

        let storage = CompStorage {
            new_gen: Box::new(new),
            insert_gen: Box::new(insert),
            delete_gen: Box::new(delete),
            get_gen: Box::new(get),
            get_by_idx_gen: Box::new(get_by_idx),
            len_gen: Box::new(len),
        };
    }
}
