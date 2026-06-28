use crate::*;

Ke!
{
    KeMemAlloc              @   fn  (layout: Layout) -> *mut u8

    KeMemFree               @   fn  (ptr: *mut u8, layout: Layout)

    KeMemAllocStack         @   fn  (size: usize) -> KeVaddr

    KeMemVirtToPhys         @   fn  (KeVaddr) -> KePaddr

    KeMemPhysToVirt         @   fn  (KePaddr) -> KeVaddr
}
