use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct KvStore {
    pub key_value: HashMap<String, String>,
}
impl KvStore {
    pub fn new() -> Self {
        Self {
            key_value: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        self.key_value.insert(key, value)
    }

    pub fn get(&self, key: String) -> Option<String> {
        match self.key_value.get(&key) {
            Some(key) => Some(key.to_owned()),
            None => None,
        }
    }
    pub fn remove(&mut self, key: String) -> Option<String> {
        match self.key_value.remove(&key) {
            Some(key) => Some(key.to_owned()),
            None => None,
        }
    }
}
