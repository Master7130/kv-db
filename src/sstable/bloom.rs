use crossbeam_skiplist::SkipMap;

use crate::ValueType;

pub struct Bloom(u8);

impl Bloom {
    pub fn create_filter(data: &SkipMap<String, ValueType>) -> Self {
        Self(0b0000_0000)
    }
}
