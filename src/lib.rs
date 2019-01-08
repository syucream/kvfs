extern crate fuse;
extern crate libc;

use std::ffi::OsStr;
use libc::ENOENT;
use fuse::{Filesystem, Request, ReplyData, ReplyEntry};

pub struct Kvfs;

// Super WIP
impl Filesystem for Kvfs {
    fn lookup(&mut self, _req: &Request, _parent: u64, _name: &OsStr, reply: ReplyEntry) {
        reply.error(ENOENT);
    }

    fn read(&mut self, _req: &Request, _ino: u64, _fh: u64, _offset: i64, _size: u32, reply: ReplyData) {
        reply.error(ENOENT);
    }
}
