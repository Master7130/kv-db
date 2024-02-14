use crate::{memtable::MemTable, ValueType};
use std::sync::{Arc, RwLock};

pub struct Store {
    memtable: MemTable,
}

impl Store {
    pub fn new() -> Self {
        Self {
            memtable: MemTable::new(),
        }
    }

    pub fn put(&self, key: String, value: ValueType) {
        self.memtable.put(key, value);
    }

    pub fn get(&self, key: String) -> Option<ValueType> {
        let res = match self.memtable.get(key) {
            Some(v) => Some(v),
            None => None
        };

        res
    }
}
