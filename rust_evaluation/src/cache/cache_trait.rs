pub(crate) trait Cache{
    fn insert_into_cache(&mut self, key: String, value: String);

    fn get_cache_content(&mut self, key: String) -> Option<&String>;

}