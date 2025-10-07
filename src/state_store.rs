use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct StateStore {
    inner: HashMap<String, HashMap<String, Value>>,
}

impl StateStore {
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    pub fn insert(&mut self, source: &str, key: String, value: Value) {
        let m = self.inner.entry(source.to_string()).or_default();
        m.insert(key, value);
    }

    pub fn get(&self, source: &str, key: &str) -> Option<&Value> {
        self.inner.get(source).and_then(|m| m.get(key))
    }
}
