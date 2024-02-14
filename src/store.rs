use crate::{memtable::MemTable, ValueType};
use std::sync::{Arc, RwLock};

pub struct Store {
    memtable: Arc<RwLock<MemTable>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            memtable: Arc::new(RwLock::new(MemTable::new())),
        }
    }

    pub fn put(&mut self, key: String, value: ValueType) {
        let mut memtable = self.memtable.write().unwrap();

        memtable.put(key, value);
    }

    pub fn get(&self, key: String) -> Option<ValueType> {
        let memtable = self.memtable.read().unwrap();

        let res = match memtable.get(key) {
            Some(v) => Some(v),
            None => None
        };

        res
    }
}
