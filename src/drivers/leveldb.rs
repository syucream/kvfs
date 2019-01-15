extern crate leveldb as libleveldb;

use std::path::Path;
use drivers::driver::Driver;
// use drivers::leveldb::libleveldb::database::Database;
use drivers::leveldb::libleveldb::options::Options;

// LevelDB driver for Kvfs
pub struct LevelDBDriver {
}

impl LevelDBDriver {
    pub fn new(path: &Path) -> LevelDBDriver {
        let mut options = Options::new();
        options.create_if_missing = true;

        /*
        let mut db = match Database::open(path, options) {
            Ok(db) => { db },
            Err(e) => { panic!("failed to open database: {:?}", e) }
        };
        */

        LevelDBDriver {}
    }
}

impl Driver for LevelDBDriver {
    fn read(&self, key: &str) -> &[u8] {
        // TODO implement

        &[]
    }

    fn write(&self, key: &str, value: &[u8]) -> u64 {
        // TODO implement

        0
    }

    fn exist(&self, key: &str) -> bool {
        // TODO implement

        true
    }
}
