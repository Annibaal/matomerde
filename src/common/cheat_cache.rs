use std::collections::HashMap;

pub struct Cache<K, V> {
    storage: HashMap<K, V>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
{
    pub fn new() -> Self {
        Cache {
            storage: HashMap::new(),
        }
    }
    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.storage.get(key)
    }
    pub fn insert(&mut self, key: K, value: V) {
        self.storage.insert(key, value);
    }
}
