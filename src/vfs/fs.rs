use alloc::string::String;
use alloc::sync::Arc;
use crate::vfs::{InodeId, Inode, Kind, Error};

pub trait FileSystem: Send + Sync {
    fn lookup(
        &self   ,
        dir     : InodeId,
        name    : &str
    )   ->      Option<InodeId>
    ;
    fn readdir(
        &self   ,
        dir     : InodeId,
        offset  : usize
    )   ->      Option<(String, InodeId)>
    ;
    fn read(
        &self   ,
        file    : InodeId,
        offset  : usize,
        buf     : &mut [u8]
    )   ->      Result<usize, Error>
    ;
    fn write(
        &self   ,
        file    : InodeId,
        offset  : usize,
        buf     : &[u8]
    )   ->      Result<usize, Error>
    ;
    fn truncate(
        &self   ,
        file    : InodeId,
        new_size: usize
    )   ->      Result<(), Error>
    ;
    fn unlink(
        &self   ,
        dir     : InodeId,
        name    : &str
    )   ->      Result<(), Error>
    ;
    fn link(
        &self   ,
        parent  : InodeId,
        name    : &str,
        child   : InodeId
    )   ->      Result<(), Error>
    ;
    fn new(
        &self   ,
        mb_id   : u32,
        inode   : Inode,
        kind    : Kind
    )   ->      Result<InodeId, Error>
    ;
    fn stat(
        &self   ,
        inode   : InodeId
    )   ->      Option<Inode>
    ;
    fn set_mb_id(
        &self   ,
        mb_id   : u32
    )
    ;
}

pub struct MetaBlock {
    pub id: u32,
    pub fs: Arc<dyn FileSystem>,
}

impl MetaBlock {
    pub const fn new(id: u32, fs: Arc<dyn FileSystem>) -> Self {
        MetaBlock { id, fs }
    }
}
