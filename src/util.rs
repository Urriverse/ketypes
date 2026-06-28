pub type KeStr = &'static str;

pub type KeDone = bool;

pub type KeAbstract<const N: usize> = [u8; N];

pub type KeCpuId = usize;

pub use core::{alloc::Layout, fmt::Arguments, panic::PanicInfo};
pub use alloc::{boxed::Box, collections::btree_map::BTreeMap, string::String, vec::Vec, sync::Arc};

pub(crate) macro Ke ( $( $(#[$attr:meta])* $n:ident @ $ty:ty )+ ) { $( $(#[$attr])* pub type $n = $ty; )+ }
