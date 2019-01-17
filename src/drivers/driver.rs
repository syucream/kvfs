pub trait Driver {
    fn read(&mut self, key: &[u8]) -> Option<Vec<u8>>;

    fn write(&mut self, key: &str, value: &[u8]) -> u64;

    fn exist(&mut self, key: &str) -> bool;
}
