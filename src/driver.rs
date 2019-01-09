extern crate leveldb;

use std::path::Path;
// use leveldb::database::Database;
use driver::leveldb::options::Options;

pub trait Driver {
    fn read(&self, key: &str) -> &[u8];

    fn write(&self, key: &str, value: &[u8]) -> u64;

    fn exist(&self, key: &str) -> bool;
}

// LevelDB driver for Kvfs
// TODO move to other module.
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
