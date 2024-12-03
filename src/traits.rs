pub trait Cache<K, V> {
    fn new(capacity: usize) -> Self;
    fn insert(&mut self, key: K, value: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn persist(&self, file_path: &str) -> std::io::Result<()>;
    fn load(&mut self, file_path: &str) -> std::io::Result<()>;
}