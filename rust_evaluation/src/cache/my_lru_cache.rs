use crate::cache::cache_trait::Cache;

#[derive(Debug)]
pub struct MyLruCache {
    cache_capacity: usize,
    cache_content: Vec<(String, String)>,
}


impl MyLruCache {
    pub fn new(_cache_capacity: usize) -> Self {
        MyLruCache { cache_capacity: _cache_capacity, cache_content: Vec::new() }
    }
}

impl Cache for MyLruCache {
    fn insert_into_cache(&mut self, _key: String, _value: String) {
        if self.cache_content.len() < self.cache_capacity {
            self.cache_content.insert(0, (_key, _value));
        }
    }

    fn get_cache_content(&mut self, _key: String) {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_into_cache() {
        let mut my_lru_cache: MyLruCache = MyLruCache::new(2);
        dbg!(&my_lru_cache);

        my_lru_cache.insert_into_cache("Clé".to_string(), "Valeur".to_string());
        my_lru_cache.insert_into_cache("Clé2".to_string(), "Valeur2".to_string());
        my_lru_cache.insert_into_cache("Clé3".to_string(), "Valeur3".to_string());
        dbg!(&my_lru_cache);
    }
}
