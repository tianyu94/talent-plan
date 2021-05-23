use std::collections::HashMap;

/// Key Value Store
///
/// # Examples
///
/// ```
/// let mut store = kvs::KvStore::new();
/// ```
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// New Key Value Store
    ///
    /// # Examples
    ///
    /// ```
    /// let mut store = kvs::KvStore::new();
    /// ```
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// # Examples
    ///
    /// ```
    /// let mut store = kvs::KvStore::new();
    /// store.set("key1".to_string(), "value1".to_string());
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// # Examples
    ///
    /// ```
    /// let mut store = kvs::KvStore::new();
    /// store.set("key1".to_string(), "value1".to_string());
    /// let result = store.get("key1".to_string());
    /// assert_eq!(result.unwrap(), "value1")
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        let option = self.map.get(&key);
        option.map(|s| s.to_string())
    }

    /// # Examples
    ///
    /// ```
    /// let mut store = kvs::KvStore::new();
    /// store.set("key1".to_string(), "value1".to_string());
    /// let removed = store.remove("key1".to_string());
    /// assert!(removed)
    /// ```
    pub fn remove(&mut self, key: String) -> bool {
        if self.map.contains_key(&key) {
            self.map.remove(&key);
            true
        } else {
            false
        }
    }
}
