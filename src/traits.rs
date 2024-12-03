pub trait Cache<K, V> {
    fn new(capacity: usize) -> Self;
    fn insert(&mut self, key: K, value: V);
    fn get(&mut self, key: &K) -> Option<&mut V>;
    fn move_to_head(&mut self, key: &K);
    fn persist(&self, file_path: &str) -> std::io::Result<()>;
    fn load(&mut self, file_path: &str) -> std::io::Result<()>;
}