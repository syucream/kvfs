extern crate fuse;
extern crate kvfs;

use std::env;
use std::path::Path;
use kvfs::core::Kvfs;
// use kvfs::driver::LevelDBDriver;

fn main() {
    let mt = env::args_os().nth(1).unwrap();

    // let path = Path::new("/tmp/kvfs");
    // let driver = LevelDBDriver::new(path);

    let fs = Kvfs {
    };

    fuse::mount(fs, &mt, &[]).unwrap();
}
