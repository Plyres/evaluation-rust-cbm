pub(crate) trait Cache{
    fn insert_into_cache(&mut self, _key: String, _value: String);

    fn get_cache_content(&mut self, _key: String);

}