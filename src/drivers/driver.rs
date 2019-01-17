pub trait Driver {
    fn read(&mut self, key: &[u8]) -> Option<Vec<u8>>;

    fn write(&mut self, key: &[u8], value: &[u8]) -> bool;

    fn exist(&mut self, key: &[u8]) -> bool;
}
