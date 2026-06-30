pub type KeStr = &'static str;

pub type KeDone = bool;

pub type KeCpuId = usize;

use core::ptr::addr_of;
pub use core::{alloc::Layout, fmt::Arguments, panic::PanicInfo};
pub use alloc::{boxed::Box, collections::btree_map::BTreeMap, string::String, vec::Vec, sync::Arc};

#[repr(align(1))]  // explicitly unaligned
pub struct KeAbstract<const N: usize>([u8; N]);

pub fn erase<T>(t: T) -> KeAbstract<{size_of::<T>()}> {
    unsafe {
        core::ptr::read_unaligned(addr_of!(t) as *const KeAbstract<{size_of::<T>()}>)
    }
}

pub fn to<T, const N: usize>(a: KeAbstract<N>) -> T {
    unsafe {
        core::ptr::read_unaligned(addr_of!(a) as *const T)
    }
}

pub macro hash($s:expr) {{
    const fn fnv1a64(data: &[u8]) -> u64 {
        const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
        const FNV_PRIME: u64 = 0x100000001b3;
        let mut hash = FNV_OFFSET_BASIS;
        let mut i = 0;
        while i < data.len() {
            hash ^= data[i] as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
            i += 1;
        }
        hash
    }
    fnv1a64($s)
}}
