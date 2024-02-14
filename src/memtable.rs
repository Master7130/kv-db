use crate::ValueType;

use crossbeam_skiplist::SkipMap;
use std::sync::{atomic::AtomicUsize, Arc};

pub struct MemTable {
    map: Arc<SkipMap<String, ValueType>>,
    size: Arc<AtomicUsize>,
}

impl MemTable {
    pub fn new() -> Self {
        Self {
            map: Arc::new(SkipMap::new()),
            size: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn put(&mut self, key: String, value: ValueType) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<ValueType> {
        match self.map.get(&key) {
            Some(v) => Some(v.value().clone()),
            None => None,
        }
    }
}
