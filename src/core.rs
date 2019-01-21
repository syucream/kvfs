use std::boxed::Box;
use std::collections::HashMap;
use std::ffi::OsStr;

use libc::ENOENT;
use fuse::{Filesystem, Request, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, ReplyWrite};

use drivers::driver::Driver;

// const TTL: Timestamp = Timestamp { sec: 1, nsec: 0 };

pub struct Kvfs {
    driver: Box<Driver>,
    inodes: HashMap<u64, (u64, String)>,
    lastInode: u64,
}

impl Kvfs {
    pub fn new(driver: Box<Driver>) -> Kvfs {
        Kvfs {
            driver: driver,
            inodes: HashMap::new(),
            lastInode: 0,
        }
    }
}

// Super WIP
impl Filesystem for Kvfs {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        let nameStr = name.to_str().unwrap().to_string();

        if !self.driver.exist(nameStr.as_bytes()) {
            reply.error(ENOENT);
        } else {
            self.inodes.insert(self.lastInode, (parent, nameStr));
            self.lastInode = self.lastInode + 1;
            // TODO
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, _req: &Request, _ino: u64, reply: ReplyAttr) {
        reply.error(ENOENT);
    }

    fn read(&mut self, _req: &Request, _ino: u64, _fh: u64, _offset: i64, _size: u32, reply: ReplyData) {
        reply.error(ENOENT);
    }

    fn readdir(&mut self, _req: &Request, _ino: u64, _fh: u64, _offset: i64, reply: ReplyDirectory) {
        reply.error(ENOENT);
    }

    fn write(&mut self, _req: &Request, _ino: u64, _fh: u64, _offset: i64, _data: &[u8], _flags: u32, reply: ReplyWrite) {
        reply.error(ENOENT);
    }
}
