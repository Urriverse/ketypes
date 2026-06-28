pub type KeStr = &'static str;

pub type KeDone = bool;

pub type KeCpuId = usize;

use core::ptr::addr_of;
pub use core::{alloc::Layout, fmt::Arguments, panic::PanicInfo};
pub use alloc::{boxed::Box, collections::btree_map::BTreeMap, string::String, vec::Vec, sync::Arc};

pub(crate) macro Ke ( $( $(#[$attr:meta])* $n:ident @ $ty:ty )+ ) { $( $(#[$attr])* pub type $n = $ty; )+ }

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
