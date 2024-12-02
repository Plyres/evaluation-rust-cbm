use std::collections::HashMap;
use std::hash::Hash;
use crate::cache::cache_trait::Cache;

#[derive(Debug)]
pub struct MyLruCache<K, V> {
    cache_capacity: usize,
    pub cache_content: HashMap<K, V>,
    pub key_order: Vec<K>
}

impl<K: Clone + Eq + Hash, V> MyLruCache<K, V> {
    /// Creates a new instance of LRU Cache
    /// # Arguments
    /// - `cache_capacity`: The size of the cache
    /// # Examples
    ///
    /// ```
    /// use rust_evaluation_cbm::cache::my_lru_cache::MyLruCache;
    ///
    /// let mut cache: MyLruCache<String, i32> = MyLruCache::new(1);
    /// assert_eq!(cache.len(), 0);
    /// ```
    pub fn new(cache_capacity: usize) -> Self {
        MyLruCache {
            cache_capacity,
            cache_content: HashMap::new(),
            key_order: Vec::new(),
        }
    }

    /// Returns the number of elements in the cache
    pub fn len(&self) -> usize {
        self.cache_content.len()
    }
}

impl<K, V> Cache<K, V> for MyLruCache<K, V>
    where
        K: Eq + Hash + Clone,
        V: Clone,
{
    /// Ajoute une paire clé/valeur dans le cache et gère l'ordre des paires à l'intérieur du cache
    /// # Arguments
    /// - `key` : La clé à ajouter dans le cache pour identifier la donnée
    /// - `value` : La valeur associée à la clé
    /// # Examples
    ///
    /// ```
    /// use rust_evaluation_cbm::cache::cache_trait::Cache;
    /// use rust_evaluation_cbm::cache::my_lru_cache::MyLruCache;
    ///
    /// let mut cache = MyLruCache::new(3);
    /// cache.insert_into_cache("key1".to_string(), 1);
    /// assert_eq!(cache.len(), 1);
    /// ```
    fn insert_into_cache(&mut self, key: K, value: V) {
        // Première condition qui vérifie si la clé passée en paramètre est déjà dans le cache
        if self.cache_content.contains_key(&key) {
            // Si la clé y est, on l'enlève de la liste d'ordre
            if let Some(index) = self.key_order.iter().position(|k| k == &key) {
                self.key_order.remove(index);
            }
        } else if self.cache_content.len() >= self.cache_capacity {
            // Si la limite de capacité du cache a été atteinte, on enlève la valeur la moins récemment utilisée
            if let Some(lru_key) = self.key_order.pop() {
                self.cache_content.remove(&lru_key);
            }
        }

        // J'insère la nouvelle paire clé-valeur et met à jour l'ordre
        self.cache_content.insert(key.clone(), value);
        self.key_order.insert(0, key);
    }

    /// Récupère la valeur du cache associée à la clé passée en paramètre et met à jour l'ordre
    /// des clés dans le cache
    /// # Arguments
    /// - `key` : La clé associée à la valeur que l'on veut récupérer dans le cache
    /// # Examples
    ///
    /// ```
    /// use rust_evaluation_cbm::cache::cache_trait::Cache;
    /// use rust_evaluation_cbm::cache::my_lru_cache::MyLruCache;
    ///
    /// let mut cache = MyLruCache::new(1);
    /// cache.insert_into_cache(1, "key1".to_string());
    /// assert_eq!(cache.get_cache_content(&1), Some(&"key1".to_string()));
    /// ```
    fn get_cache_content(&mut self, key: &K) -> Option<&V> {
        if let Some(index) = self.key_order.iter().position(|k| k == key) {
            // Move the accessed key to the front of the order list
            let k = self.key_order.remove(index);
            self.key_order.insert(0, k);
            self.cache_content.get(key)
        } else {
            None
        }
    }
}