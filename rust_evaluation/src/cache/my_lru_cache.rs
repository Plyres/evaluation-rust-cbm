use crate::cache::cache_trait::Cache;

#[derive(Debug)]
pub struct MyLruCache {
    cache_capacity: usize,
    cache_content: Vec<(String, String)>,
}


impl MyLruCache {
    pub fn new(cache_capacity: usize) -> Self {
        MyLruCache { cache_capacity, cache_content: Vec::new() }
    }
}

impl Cache for MyLruCache {
    fn insert_into_cache(&mut self, key: String, value: String) {
        if let Some(index) = self.cache_content.iter().position(|(k, _)| k == &key) {
           self.cache_content.remove(index);
        } else if self.cache_content.len() >= self.cache_capacity {
            self.cache_content.pop();
        }

        self.cache_content.insert(0, (key, value));
    }

    fn get_cache_content(&mut self, key: String) -> Option<&String> {
        if let Some(index) = self.cache_content.iter().position(|(k, _)| k == &key) {
            let entry = self.cache_content.remove(index);
            self.cache_content.insert(0, entry);
            Some(&self.cache_content[0].1)
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut cache = MyLruCache::new(3);
        cache.insert_into_cache("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get_cache_content("key1".to_string()), Some(&"value1".to_string()));
    }

    #[test]
    fn test_capacity_limit() {
        let mut cache = MyLruCache::new(2);
        cache.insert_into_cache("key1".to_string(), "value1".to_string());
        cache.insert_into_cache("key2".to_string(), "value2".to_string());
        cache.insert_into_cache("key3".to_string(), "value3".to_string());
        assert_eq!(cache.get_cache_content("key1".to_string()), None);
        assert_eq!(cache.get_cache_content("key2".to_string()), Some(&"value2".to_string()));
        assert_eq!(cache.get_cache_content("key3".to_string()), Some(&"value3".to_string()));
    }

    #[test]
    fn test_update_existing_key() {
        let mut cache = MyLruCache::new(2);
        cache.insert_into_cache("key1".to_string(), "value1".to_string());
        cache.insert_into_cache("key2".to_string(), "value2".to_string());
        cache.insert_into_cache("key1".to_string(), "new_value1".to_string());
        assert_eq!(cache.get_cache_content("key1".to_string()), Some(&"new_value1".to_string()));
        assert_eq!(cache.get_cache_content("key2".to_string()), Some(&"value2".to_string()));
    }

    #[test]
    fn test_lru_order() {
        let mut cache = MyLruCache::new(3);
        cache.insert_into_cache("key1".to_string(), "value1".to_string());
        cache.insert_into_cache("key2".to_string(), "value2".to_string());
        cache.insert_into_cache("key3".to_string(), "value3".to_string());
        cache.get_cache_content("key1".to_string());
        cache.insert_into_cache("key4".to_string(), "value4".to_string());
        assert_eq!(cache.get_cache_content("key2".to_string()), None);
        assert_eq!(cache.get_cache_content("key1".to_string()), Some(&"value1".to_string()));
        assert_eq!(cache.get_cache_content("key3".to_string()), Some(&"value3".to_string()));
        assert_eq!(cache.get_cache_content("key4".to_string()), Some(&"value4".to_string()));
    }

    #[test]
    fn test_index_change_on_get() {
        let mut cache = MyLruCache::new(3);
        cache.insert_into_cache("key1".to_string(), "value1".to_string());
        cache.insert_into_cache("key2".to_string(), "value2".to_string());
        cache.insert_into_cache("key3".to_string(), "value3".to_string());

        // Accès à "key1" pour le déplacer en tête de cache
        cache.get_cache_content("key1".to_string());

        // Vérification que "key1" est maintenant en première position
        assert_eq!(cache.cache_content[0].0, "key1");

        // Vérification que l'ordre des autres éléments a changé
        assert_eq!(cache.cache_content[1].0, "key3");
        assert_eq!(cache.cache_content[2].0, "key2");
    }
}