pub(crate) trait Cache<K,V>{
    fn insert_into_cache(&mut self, key: K, value: V);

    fn get_cache_content(&mut self, key: K) -> Option<&V>;

}