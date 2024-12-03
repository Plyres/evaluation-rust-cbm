pub mod cache;

#[cfg(test)]
mod integration_tests {
    use crate::cache::cache_trait::Cache;

    #[test]
    fn test_insert_and_get() {
        let mut cache = crate::cache::my_lru_cache::MyLruCache::new(3);
        cache.insert_into_cache(String::from("key1"), 1);
        assert_eq!(cache.get_cache_content(&String::from("key1")), Some(&1));
    }

    #[test]
    fn test_capacity_limit() {
        let mut cache = crate::cache::my_lru_cache::MyLruCache::new(2);
        cache.insert_into_cache(1, String::from("value1"));
        cache.insert_into_cache(2, String::from("value2"));
        cache.insert_into_cache(3, String::from("value3"));
        assert_eq!(cache.get_cache_content(&1), None);
        assert_eq!(cache.get_cache_content(&2), Some(&String::from("value2")));
        assert_eq!(cache.get_cache_content(&3), Some(&String::from("value3")));
    }

    #[test]
    fn test_update_existing_key() {
        let mut cache = crate::cache::my_lru_cache::MyLruCache::new(2);
        cache.insert_into_cache(String::from("key1"), String::from("value1"));
        cache.insert_into_cache(String::from("key2"), String::from("value2"));
        cache.insert_into_cache(String::from("key1"), "new_value1".to_string());
        assert_eq!(cache.get_cache_content(&String::from("key1")), Some(&"new_value1".to_string()));
        assert_eq!(cache.get_cache_content(&String::from("key2")), Some(&String::from("value2")));
    }

    #[test]
    fn test_lru_order() {
        let mut cache = crate::cache::my_lru_cache::MyLruCache::new(3);
        cache.insert_into_cache(String::from("key1"), String::from("value1"));
        cache.insert_into_cache(String::from("key2"), String::from("value2"));
        cache.insert_into_cache(String::from("key3"), String::from("value3"));
        cache.get_cache_content(&String::from("key1"));
        cache.insert_into_cache("key4".to_string(), "value4".to_string());
        assert_eq!(cache.get_cache_content(&String::from("key2")), None);
        assert_eq!(cache.get_cache_content(&String::from("key1")), Some(&String::from("value1")));
        assert_eq!(cache.get_cache_content(&String::from("key3")), Some(&String::from("value3")));
        assert_eq!(cache.get_cache_content(&"key4".to_string()), Some(&"value4".to_string()));
    }

    #[test]
    fn test_index_change_on_get() {
        let mut cache = crate::cache::my_lru_cache::MyLruCache::new(3);
        cache.insert_into_cache(String::from("key1"), String::from("value1"));
        cache.insert_into_cache(String::from("key2"), String::from("value2"));
        cache.insert_into_cache(String::from("key3"), String::from("value3"));

        // Accès à "key1" pour le déplacer en tête de cache
        cache.get_cache_content(&String::from("key1"));

        // Vérification que "key1" est maintenant en première position
        assert_eq!(cache.key_order[0], "key1");

        // Vérification que l'ordre des autres éléments a changé
        assert_eq!(cache.key_order[1], "key3");
        assert_eq!(cache.key_order[2], "key2");
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::cache::cache_trait::Cache;
    use crate::cache::my_lru_cache::MyLruCache;

    #[test]
    fn test_insert_into_cache(){
        let mut cache = MyLruCache::new(2);
        cache.insert_into_cache(1, String::from("key1"));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_insert_into_cache_capacity(){
        let mut cache = MyLruCache::new(3);
        cache.insert_into_cache(String::from("key1"), 1);
        cache.insert_into_cache(String::from("key2"), 2);
        cache.insert_into_cache(String::from("key3"), 3);
        cache.insert_into_cache(String::from("key4"), 4);
        assert_eq!(cache.len(), 3);
    }


    #[test]
    fn test_get_cache_content(){
        let mut cache = MyLruCache::new(1);
        cache.cache_content.insert(1,String::from("key1"));
        cache.key_order.push(1);
        assert_eq!(cache.get_cache_content(&1), Some(&String::from("key1")));
    }

}