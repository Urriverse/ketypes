use alloc::string::String;
use alloc::sync::Arc;
use crate::vfs::{KeInodeId, KeInode, Kind, KeFsError};

pub trait KeFileSystem: Send + Sync {
    fn lookup(
        &self   ,
        dir     : KeInodeId,
        name    : &str
    )   ->      Option<KeInodeId>
    ;
    fn readdir(
        &self   ,
        dir     : KeInodeId,
        offset  : usize
    )   ->      Option<(String, KeInodeId)>
    ;
    fn read(
        &self   ,
        file    : KeInodeId,
        offset  : usize,
        buf     : &mut [u8]
    )   ->      Result<usize, KeFsError>
    ;
    fn write(
        &self   ,
        file    : KeInodeId,
        offset  : usize,
        buf     : &[u8]
    )   ->      Result<usize, KeFsError>
    ;
    fn truncate(
        &self   ,
        file    : KeInodeId,
        new_size: usize
    )   ->      Result<(), KeFsError>
    ;
    fn unlink(
        &self   ,
        dir     : KeInodeId,
        name    : &str
    )   ->      Result<(), KeFsError>
    ;
    fn link(
        &self   ,
        parent  : KeInodeId,
        name    : &str,
        child   : KeInodeId
    )   ->      Result<(), KeFsError>
    ;
    fn new(
        &self   ,
        mb_id   : u32,
        inode   : KeInode,
        kind    : Kind
    )   ->      Result<KeInodeId, KeFsError>
    ;
    fn stat(
        &self   ,
        inode   : KeInodeId
    )   ->      Option<KeInode>
    ;
    fn set_mb_id(
        &self   ,
        mb_id   : u32
    )
    ;
}

pub type KeMetaBlockId = u32;

pub struct KeMetaBlock {
    pub id: KeMetaBlockId,
    pub fs: Arc<dyn KeFileSystem>,
}

impl KeMetaBlock {
    pub const fn new(id: KeMetaBlockId, fs: Arc<dyn KeFileSystem>) -> Self {
        KeMetaBlock { id, fs }
    }
}
