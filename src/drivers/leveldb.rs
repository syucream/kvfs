extern crate rusty_leveldb;

use self::rusty_leveldb::{DB, Options};

use drivers::driver::Driver;

// LevelDB driver for Kvfs
pub struct LevelDBDriver {
    database: DB,
}

impl LevelDBDriver {
    pub fn new(path: &str) -> LevelDBDriver {
        let opts = Options::default();

        let mut db = DB::open(path, opts).unwrap();

        LevelDBDriver {
            database: db,
        }
    }
}

impl Driver for LevelDBDriver {
    fn read(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        self.database.get(key)
    }

    fn write(&mut self, key: &str, value: &[u8]) -> u64 {
        // TODO implement

        0
    }

    fn exist(&mut self, key: &str) -> bool {
        // TODO implement

        true
    }
}
