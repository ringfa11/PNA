//! simple crate
#![deny(missing_docs)]
use std::collections::HashMap;

/// KvStore store k/v pairs in a HashMap
pub struct KvStore {
    store: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        KvStore::new()
    }
}

impl KvStore {
    /// create a new store
    pub fn new() -> KvStore {
        let store = HashMap::new();
        KvStore { store }
    }
    /// set k/v pair
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }
    /// get value by key
    pub fn get(&mut self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }
    /// remove k/v pair
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
