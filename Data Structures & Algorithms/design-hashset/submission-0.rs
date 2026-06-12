use std::{
    collections::BTreeSet,
    hash::{DefaultHasher, Hash},
};

struct MyHashSet {
    hasher: DefaultHasher,
    set: BTreeSet<i32>,
}

impl MyHashSet {
    pub fn new() -> Self {
        Self {
            hasher: DefaultHasher::new(),
            set: BTreeSet::new(),
        }
    }

    pub fn add(&mut self, key: i32) {
        key.hash(&mut self.hasher);
        self.set.insert(key);
    }

    pub fn remove(&mut self, key: i32) {
        key.hash(&mut self.hasher);
        self.set.remove(&key);
    }

    pub fn contains(&mut self, key: i32) -> bool {
        key.hash(&mut self.hasher);
        self.set.contains(&key)
    }
}