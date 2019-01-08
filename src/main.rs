extern crate fuse;
extern crate kvfs;

use std::env;
use kvfs::Kvfs;

fn main() {
    let mt = env::args_os().nth(1).unwrap();

    fuse::mount(Kvfs, &mt, &[]).unwrap();
}