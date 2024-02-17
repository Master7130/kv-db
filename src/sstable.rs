mod bloom;

use bloom::Bloom;
use crossbeam_skiplist::SkipMap;
use std::{
    collections::hash_map::Values,
    fmt::format,
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
};

use crate::ValueType;

pub struct SSTable {
    id: usize,
    pub file: FileObject,
    pub bloom: Bloom,
    blocks: Vec<Block>,
}

impl SSTable {
    pub fn new(id: usize, data: &SkipMap<String, ValueType>) -> Self {
        // TODO: Sort keys

        let mut file = FileObject::new(id);
        let blocks = file.write_data(data);

        let bloom = Bloom::create_filter(data);

        Self {
            id,
            file,
            bloom,
            blocks,
        }
    }

    pub fn search(&mut self, key: &String) -> Option<ValueType> {
        let present = self.bloom.check_filter(key);

        // TODO: Refactor to binary search
        let mut res: Option<Block> = None;
        for b in self.blocks.iter() {
            if b.key == *key {
                res = Some(b.clone());

                break;
            }
        }

        match res {
            Some(b) => {
                let val = self.file.read_block(&b);

                Some(val)
            }
            None => None,
        }
    }
}

pub struct FileObject(File);

impl FileObject {
    pub fn new(id: usize) -> Self {
        let file = OpenOptions::new().read(true)
        .write(true)
        .create(true)
        .open(format!("{id}.db"))
        .unwrap();

        Self(file)
    }

    pub fn write_data(&mut self, data: &SkipMap<String, ValueType>) -> Vec<Block> {
        let mut idx = 0;

        let mut blocks: Vec<Block> = Vec::new();
        for i in data.into_iter() {
            let block_item = format!("{}{}", i.key(), i.value());
            let block_size = block_item.as_bytes().len();

            let block = Block {
                start: idx + i.key().as_bytes().len(),
                end: idx + block_size,
                key: i.key().to_string(),
            };
            blocks.push(block);

            idx += block_size;

            self.0.write_all(block_item.as_bytes());
        }

        blocks
    }

    pub fn read_block(&mut self, block: &Block) -> ValueType {
        self.0.seek(SeekFrom::Start(block.start as u64));

        let mut buf = vec![0; block.end - block.start];

        self.0.read_exact(&mut buf).unwrap();

        ValueType::String(String::from_utf8(buf).unwrap_or_default())
    }
}

/// Represents one key-value pair in the file
#[derive(Clone)]
pub struct Block {
    start: usize,
    end: usize,
    key: String,
}
