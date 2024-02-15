mod bloom;

use bloom::Bloom;
use crossbeam_skiplist::SkipMap;
use std::{collections::hash_map::Values, fmt::format, fs::File, io::Write};

use crate::ValueType;

pub struct SSTable {
    id: usize,
    pub file: FileObject,
    pub bloom: Bloom,
}

impl SSTable {
    pub fn new(id: usize, data: &SkipMap<String, ValueType>) -> Self {
        // TODO: Sort keys

        let mut file = FileObject::new(id);
        file.write_data(data);

        let bloom = Bloom::create_filter(data);

        Self { id, file, bloom }
    }
}

pub struct FileObject(File);

impl FileObject {
    pub fn new(id: usize) -> Self {
        println!("Creating file");
        Self(File::create(format!("{id}.txt")).unwrap())
    }

    pub fn write_data(&mut self, data: &SkipMap<String, ValueType>) {
        let mut idx = 0;
        for i in data.into_iter() {
            let block_item = format!("{}{}", i.key(), i.value());
            let block_size = block_item.as_bytes().len();

            let block = Block {
                start: idx,
                end: idx + block_size,
                key: i.key().to_string(),
            };

            idx += block_size;

            self.0.write_all(block_item.as_bytes());
        }
    }
}

/// Represents one key-value pair in the file
pub struct Block {
    start: usize,
    end: usize,
    key: String,
}
