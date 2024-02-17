use crate::{memtable::MemTable, sstable::SSTable, ValueType};
use std::sync::{Arc, RwLock};

pub struct Store {
    memtable: MemTable,
    sstables: RwLock<Vec<SSTable>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            memtable: MemTable::new(),
            sstables: RwLock::new(Vec::new()),
        }
    }

    pub fn put(&self, key: String, value: ValueType) {
        let size = self.memtable.put(key, value);

        if size >= 2 {
            let num_tables = self.sstables.read().unwrap().len();
            let sstable = self.memtable.flush(num_tables);

            let mut sstable_vec = self.sstables.write().unwrap();
            sstable_vec.push(sstable);
        }
    }

    pub fn get(&self, key: String) -> Option<ValueType> {
        let res = match self.memtable.get(key.clone()) {
            Some(v) => Some(v),
            None => {
                let mut tables = self.sstables.write().unwrap();
                let table_len = tables.len();
                for i in 0..table_len {
                    match tables[i].search(&key) {
                        Some(v) => return Some(v),
                        None => continue,
                    };
                }

                None
            },
        };

        res
    }
}
