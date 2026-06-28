use crate::*;

Ke!
{
    KeFsLookup              @   fn  (mb: &KeMetaBlock, dir: KeInodeId, name: &str) -> Option<KeInodeId>

    KeFsReadDir             @   fn  (mb: &KeMetaBlock, dir: KeInodeId, offset: usize) -> Option<(String, KeInodeId)>

    KeFsRead                @   fn  (mb: &KeMetaBlock, file: KeInodeId, offset: usize, buf: &mut [u8]) -> Result<usize, KeFsError>

    KeFsWrite               @   fn  (mb: &KeMetaBlock, file: KeInodeId, offset: usize, buf: &[u8]) -> Result<usize, KeFsError>

    KeFsTrucate             @   fn  (mb: &KeMetaBlock, file: KeInodeId, new_size: usize) -> Result<(), KeFsError>

    KeFsLink                @   fn  (mb: &KeMetaBlock, parent: KeInodeId, name: &str, child: KeInodeId) -> Result<(), KeFsError>

    KeFsUnlink              @   fn  (mb: &KeMetaBlock, dir: KeInodeId, name: &str) -> Result<(), KeFsError>

    KeFsObjectNew           @   fn  (mb: &KeMetaBlock, inode: KeInode, kind: Kind) -> Result<KeInodeId, KeFsError>

    KeFsObjectStat          @   fn  (mb: &KeMetaBlock, inode: KeInodeId) -> Option<KeInode>

    KeFsObjectIsMountPoint  @   fn  (id: KeInodeId) -> bool

    KeFsResolve             @   fn  (path: &str) -> Result<(KeInodeId, Arc<KeMetaBlock>), KeFsError>

    KeFsListDir             @   fn  (mb: &KeMetaBlock, dir: KeInodeId) -> BTreeMap<String, KeInodeId>

    KeFsReadToString        @   fn  (mb: &KeMetaBlock, file: KeInodeId) -> Result<String, KeFsError>

    KeFsMetaBlockRegister   @   fn  (fs: Arc<dyn KeFileSystem>) -> KeMetaBlockId

    KeFsMount               @   fn  (name: String, mb: KeMetaBlockId) -> Option<KeInodeId>
}
