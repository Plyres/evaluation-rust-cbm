use crate::cache::cache_trait::Cache;

#[derive(Debug)]
pub struct MyLruCache<K, V> {
    cache_capacity: usize,
    cache_content: Vec<(K, V)>,
}


impl <K: Clone + Eq, V> MyLruCache<K, V>  {
    pub fn new(cache_capacity: usize) -> Self {
        MyLruCache { cache_capacity, cache_content: Vec::new() }
    }
}

impl <K, V> Cache<K, V> for MyLruCache<K, V> where
    K: Eq + Clone,
    V: Clone,{
    fn insert_into_cache(&mut self, key: K, value: V) {
        //Première condition qui vérifie si la clé passé en paramètre est déjà dans le cache
        if let Some(index) = self.cache_content.iter().position(|(k, _)| k == &key) {
            // Si elle l'est on l'enlève pour mettre à jour sa valeur avec celle passée en paramètre
            // et placer cette paire clé-valeur en première dans le cache
           self.cache_content.remove(index);
        }
        //On vérifie si la limite de capacité du cache a été atteinte si oui, on enlève la valeur la moins récemment utilisé
        else if self.cache_content.len() >= self.cache_capacity {
            self.cache_content.pop();
        }

        self.cache_content.insert(0, (key, value));
    }

    fn get_cache_content(&mut self, key: K) -> Option<&V> {
        if let Some(index) = self.cache_content.iter().position(|(k, _)| k == &key) {
            let entry = self.cache_content.remove(index);
            self.cache_content.insert(0, entry);
            // Récupére la valeur associé à la clé passé en paramètre
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
        cache.insert_into_cache("key1".to_string(), 1);
        assert_eq!(cache.get_cache_content("key1".to_string()), Some(&1));
    }

    #[test]
    fn test_capacity_limit() {
        let mut cache = MyLruCache::new(2);
        cache.insert_into_cache(1, "value1".to_string());
        cache.insert_into_cache(2, "value2".to_string());
        cache.insert_into_cache(3, "value3".to_string());
        assert_eq!(cache.get_cache_content(1), None);
        assert_eq!(cache.get_cache_content(2), Some(&"value2".to_string()));
        assert_eq!(cache.get_cache_content(3), Some(&"value3".to_string()));
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