use crate::*;

Ke!
{
    KePagingMap             @   fn  (va: KeVaddr, pa: KePaddr, size: usize, flags: KeEntryFlags) -> Result<(), KeStr>

    KePagingRemap           @   fn  (va: KeVaddr, size: usize, new_flags: KeEntryFlags) -> Result<(), KeStr>

    KePagingUnmap           @   fn  (va: KeVaddr, size: usize) -> Result<(), KeStr>

    KePagingMerge           @   fn  (start: KeVaddr, size: usize)

    KePagingQuery           @   fn  (va: KeVaddr) -> Option<(KePaddr, KeEntryFlags)>
}
