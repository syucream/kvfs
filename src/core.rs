use std::boxed::Box;
use std::ffi::OsStr;
use libc::ENOENT;
use fuse::{Filesystem, Request, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, ReplyWrite};

use drivers::driver::Driver;

pub struct Kvfs {
    driver: Box<Driver>,
}

impl Kvfs {
    pub fn new(driver: Box<Driver>) -> Kvfs {
        Kvfs {
            driver: driver,
        }
    }
}

// Super WIP
impl Filesystem for Kvfs {
    fn lookup(&mut self, _req: &Request, _parent: u64, _name: &OsStr, reply: ReplyEntry) {
        reply.error(ENOENT);
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
