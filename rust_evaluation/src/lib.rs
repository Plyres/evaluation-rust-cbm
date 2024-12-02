pub mod cache;

#[cfg(test)]
mod tests {
    use crate::cache::cache_trait::Cache;

    #[test]
    fn test_insert_and_get() {
        let mut cache = crate::cache::my_lru_cache::MyLruCache::new(3);
        cache.insert_into_cache("key1".to_string(), 1);
        assert_eq!(cache.get_cache_content(&"key1".to_string()), Some(&1));
    }

    #[test]
    fn test_capacity_limit() {
        let mut cache = crate::cache::my_lru_cache::MyLruCache::new(2);
        cache.insert_into_cache(1, "value1".to_string());
        cache.insert_into_cache(2, "value2".to_string());
        cache.insert_into_cache(3, "value3".to_string());
        assert_eq!(cache.get_cache_content(&1), None);
        assert_eq!(cache.get_cache_content(&2), Some(&"value2".to_string()));
        assert_eq!(cache.get_cache_content(&3), Some(&"value3".to_string()));
    }

    #[test]
    fn test_update_existing_key() {
        let mut cache = crate::cache::my_lru_cache::MyLruCache::new(2);
        cache.insert_into_cache("key1".to_string(), "value1".to_string());
        cache.insert_into_cache("key2".to_string(), "value2".to_string());
        cache.insert_into_cache("key1".to_string(), "new_value1".to_string());
        assert_eq!(cache.get_cache_content(&"key1".to_string()), Some(&"new_value1".to_string()));
        assert_eq!(cache.get_cache_content(&"key2".to_string()), Some(&"value2".to_string()));
    }

    #[test]
    fn test_lru_order() {
        let mut cache = crate::cache::my_lru_cache::MyLruCache::new(3);
        cache.insert_into_cache("key1".to_string(), "value1".to_string());
        cache.insert_into_cache("key2".to_string(), "value2".to_string());
        cache.insert_into_cache("key3".to_string(), "value3".to_string());
        cache.get_cache_content(&"key1".to_string());
        cache.insert_into_cache("key4".to_string(), "value4".to_string());
        assert_eq!(cache.get_cache_content(&"key2".to_string()), None);
        assert_eq!(cache.get_cache_content(&"key1".to_string()), Some(&"value1".to_string()));
        assert_eq!(cache.get_cache_content(&"key3".to_string()), Some(&"value3".to_string()));
        assert_eq!(cache.get_cache_content(&"key4".to_string()), Some(&"value4".to_string()));
    }

    #[test]
    fn test_index_change_on_get() {
        let mut cache = crate::cache::my_lru_cache::MyLruCache::new(3);
        cache.insert_into_cache("key1".to_string(), "value1".to_string());
        cache.insert_into_cache("key2".to_string(), "value2".to_string());
        cache.insert_into_cache("key3".to_string(), "value3".to_string());

        // Accès à "key1" pour le déplacer en tête de cache
        cache.get_cache_content(&"key1".to_string());

        // Vérification que "key1" est maintenant en première position
        assert_eq!(cache.key_order[0], "key1");

        // Vérification que l'ordre des autres éléments a changé
        assert_eq!(cache.key_order[1], "key3");
        assert_eq!(cache.key_order[2], "key2");
    }
}