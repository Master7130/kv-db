use crate::ValueType;

use crossbeam_skiplist::SkipMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct Bloom(u8, u8);

impl Bloom {
    pub fn create_filter(data: &SkipMap<String, ValueType>) -> Self {
        // let hasher_factory = |salt: u64| -> Box<dyn Fn(&String) -> u8 + Send> {
        //     Box::new(move |s: &String| -> u8 {
        //         let mut hasher = DefaultHasher::new();
        //         (s.as_str(), salt).hash(&mut hasher);
        //         let hash = hasher.finish();
        //
        //         (hash % 8) as u8
        //     })
        // };

        let mut filter: u8 = 0b0000_0000;

        for i in data.into_iter() {
            let key = i.key();

            for i in 1..3 {
                let res = hasher(i, key);
                filter |= (1 << res);
            }
        }

        Self(filter, 2)
    }

    pub fn check_filter(&self, key: &String) -> bool {
        true
    }
}

fn hasher(salt: u64, s: &String) -> u8 {
    let mut hasher = DefaultHasher::new();
    (s.as_str(), salt).hash(&mut hasher);
    let hash = hasher.finish();

    (hash % 8) as u8
}
