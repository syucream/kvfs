extern crate fuse;
extern crate kvfs;

use std::boxed::Box;
use std::env;
use std::path::Path;
use kvfs::core::Kvfs;
use kvfs::drivers::leveldb::LevelDBDriver;

fn main() {
    let mt = env::args_os().nth(1).unwrap();

    let path = Path::new("/tmp/kvfs");
    let driver = Box::new(LevelDBDriver::new(path));
    let fs = Kvfs::new(driver);

    fuse::mount(fs, &mt, &[]).unwrap();
}
