use std::hash::{DefaultHasher, Hasher};

const CAPACITY: usize = 128;

struct MyHashMap(Vec<Vec<(i32, i32)>>);

impl MyHashMap {
    pub fn new() -> Self {
        Self(vec![Vec::new(); CAPACITY])
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let mut hasher = DefaultHasher::new();
        hasher.write_i32(key);
        let vec = &mut self.0[hasher.finish() as usize % CAPACITY];
        for (k, v) in &mut *vec {
            if *k == key {
                *v = value;
                return;
            }
        }
        vec.push((key, value));
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let mut hasher = DefaultHasher::new();
        hasher.write_i32(key);
        for (k, v) in &self.0[hasher.finish() as usize % CAPACITY] {
            if *k == key {
                return *v;
            }
        }
        -1
    }

    pub fn remove(&mut self, key: i32) {
        let mut hasher = DefaultHasher::new();
        hasher.write_i32(key);
        let vec = &mut self.0[hasher.finish() as usize % CAPACITY];
        for i in 0..vec.len() {
            if vec[i].0 == key {
                vec.remove(i);
            }
        }
    }
}