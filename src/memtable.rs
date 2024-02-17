use crate::sstable::SSTable;
use crate::ValueType;

use crossbeam_skiplist::SkipMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

pub struct MemTable {
    map: SkipMap<String, ValueType>,
    size: Arc<AtomicUsize>,
}

impl MemTable {
    pub fn new() -> Self {
        Self {
            map: SkipMap::new(),
            size: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn put(&self, key: String, value: ValueType) -> usize {
        let size = self.size.clone();
        size.store(size.load(Ordering::SeqCst) + 1, Ordering::SeqCst);

        self.map.insert(key, value);

        size.load(Ordering::SeqCst)
    }

    pub fn get(&self, key: String) -> Option<ValueType> {
        match self.map.get(&key) {
            Some(v) => Some(v.value().clone()),
            None => None,
        }
    }

    pub fn flush(&self, id: usize) -> SSTable {
        let sstable = SSTable::new(id, &self.map);

        self.map.clear();

        let size = self.size.clone();
        size.store(0, Ordering::SeqCst);

        sstable
    }
}
