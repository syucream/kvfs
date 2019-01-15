pub trait Driver {
    fn read(&self, key: &str) -> &[u8];

    fn write(&self, key: &str, value: &[u8]) -> u64;

    fn exist(&self, key: &str) -> bool;
}
