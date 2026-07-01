#![no_std]
#![allow(incomplete_features)]  // for generic_const_exprs
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(decl_macro)]

extern crate alloc;
#[macro_use] extern crate apaque;

pub extern crate linkme;

pub mod sync;
pub mod mon;
pub mod vfs;
pub mod ebus;
pub mod kdm;
pub mod paging;
pub mod km;
pub mod util;
pub mod sched;
pub mod dev;

pub use km::*;
pub use mon::*;
pub use sync::*;
pub use vfs::*;
pub use ebus::*;
pub use kdm::*;
pub use paging::*;
pub use util::*;
pub use sched::*;
pub use dev::*;

pub use paste::paste;

#[repr(C)] pub struct ImExport(pub *const (), pub u64);
#[repr(C)] pub struct Kexport(pub *const (), pub u64, pub *const str);

unsafe impl core::marker::Send for ImExport {}
unsafe impl core::marker::Sync for ImExport {}

unsafe impl core::marker::Send for Kexport {}
unsafe impl core::marker::Sync for Kexport {}

impl ImExport {
    #[inline(always)] pub fn address(&self) -> usize { self.0 as usize }
    #[inline(always)] pub fn version(&self) -> (u32, u32) { ((self.1 >> 32) as u32, self.1 as u32) }
    #[inline(always)] pub unsafe fn borrow<T>(&self) -> &'static mut T { unsafe { (self.0 as *mut T).as_mut_unchecked() } }
}

impl Kexport {
    #[inline(always)] pub fn address(&self) -> usize { self.0 as usize }
    #[inline(always)] pub fn name(&self) -> &'static str { unsafe { self.2.as_ref_unchecked() } }
    #[inline(always)] pub fn version(&self) -> (u32, u32) { ((self.1 >> 32) as u32, self.1 as u32) }
    #[inline(always)] pub unsafe fn borrow<T>(&self) -> &'static mut T { unsafe { (self.0 as *mut T).as_mut_unchecked() } }
}

pub const fn parse_version(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut dot_pos = len;
    let (mut major, mut minor, mut i, mut j) = (0, 0, 0, 0);

    while i < len {
        if bytes[i] == b'.' {
            dot_pos = i;
            break
        }
        i += 1
    }
    while j < dot_pos {
        let digit = bytes[j];
        if digit < b'0' || digit > b'9' { panic!("invalid") }
        major = major * 10 + (digit - b'0') as u32;
        j+=1
    }
    if dot_pos < len {
        let mut k = dot_pos + 1;
        while k < len {
            let digit = bytes[k];
            if digit < b'0' || digit > b'9' {
                return 0
            }
            minor = minor * 10 + ( digit - b'0') as u32;
            k+=1
        }
    }
    
    (major as u64) << 32 | minor as u64
}

#[macro_export]
macro_rules! Import {
    ( $i:ident => $n:ident, since kernel $x:literal ) => {
        $crate::paste!(
            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = concat!("_Ki", stringify!($n)))]
            static [< Ki $n >]: $crate::ImExport = $crate::ImExport($i as *const (), $crate::parse_version(stringify!($x)));
        );
    };

    ( $n:ident, since $x:literal ) => {
        $crate::paste!(
            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = concat!("_Ki", stringify!($n)))]
            static [< Ki $n >]: $crate::ImExport = $crate::ImExport($n as *const (), $crate::parse_version(stringify!($x)));
        );
    };
    ( $i:ident => $n:ident, since $x:literal ) => {
        $crate::paste!(
            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = concat!("_Mi", stringify!($n)))]
            static [< Mi $n >]: $crate::ImExport = $crate::ImExport($i as *const (), $crate::parse_version(stringify!($x)));
        );
    };
    ( $n:ident, since $x:literal ) => {
        $crate::paste!(
            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = concat!("_Mi", stringify!($n)))]
            static [< Mi $n >]: $crate::ImExport = $crate::ImExport($n as *const (), $crate::parse_version(stringify!($x)));
        );
    };
    ( $(#[$attr:meta])* $vis:vis fn $n:ident ( $($name:ident : $aty:ty),* ) $( -> $rty:ty )? where kernel $x:literal ) => {
        $crate::paste!(
            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = concat!("Ki", stringify!($n)))]
            static [< _ $n >]: $crate::ImExport = $crate::ImExport(0 as *const (), $crate::parse_version(stringify!($x)));

            $(#[$attr])*
            #[allow(non_snake_case)]
            #[inline(always)]
            $vis fn [< $n >]( $( $name : $aty ),* ) $( -> $rty )? {
                (unsafe{core::mem::transmute::<_, fn ( $( $name : $aty ),* ) $( -> $rty )?>([< _ $n >].0 )})( $( $name ),* )
            }
        );
    };
    ( $(#[$attr:meta])* $vis:vis fn $n:ident ( $($name:ident : $aty:ty),* ) $( -> $rty:ty )? where kernel $x:literal $b:block ) => {
        $crate::paste!(
            #[allow(non_snake_case)]
            fn [< __stub_ $n >]( $( $name : $aty ),* ) $( -> $rty )? { $b }

            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = concat!("Ki", stringify!($n)))]
            static [< _ $n >]: $crate::ImExport = $crate::ImExport([< __stub_ $n >] as *const (), $crate::parse_version(stringify!($x)));

            $(#[$attr])*
            #[allow(non_snake_case)]
            #[inline(always)]
            $vis fn [< $n >]( $( $name : $aty ),* ) $( -> $rty )? {
                (unsafe{core::mem::transmute::<_, fn ( $( $name : $aty ),* ) $( -> $rty )?>([< _ $n >].0 )})( $( $name ),* )
            }
        );
    };
    ( $(#[$attr:meta])* $vis:vis fn $n:ident ( $($name:ident : $aty:ty),* ) $( -> $rty:ty )? where $x:literal $b:block ) => {
        $crate::paste!(
            #[allow(non_snake_case)]
            fn [< __stub_ $n >]( $( $name : $aty ),* ) $( -> $rty )? { $b }

            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = concat!("Mi", stringify!($n)))]
            static [< _ $n >]: $crate::ImExport = $crate::ImExport([< __stub_ $n >] as *const (), $crate::parse_version(stringify!($x)));

            $(#[$attr])*
            #[allow(non_snake_case)]
            #[inline(always)]
            $vis fn [< $n >]( $( $name : $aty ),* ) $( -> $rty )? {
                (unsafe{core::mem::transmute::<_, fn ( $( $name : $aty ),* ) $( -> $rty )?>([< _ $n >].0 )})( $( $name ),* )
            }
        );
    };
}

#[macro_export]
macro_rules! Export {
    ($($i:tt)::+ => $n:ident, since kernel $x:literal) => {
        $crate::paste!(
            #[used]
            #[allow(non_upper_case_globals)]
            #[linkme::distributed_slice(crate::KMI_TABLE)]
            static [< Ke $n >]: $crate::Kexport = $crate::Kexport($($i)::+ as *const (), $crate::parse_version(stringify!($x)), stringify!($n));
        );
    };
    ($($i:tt)::+, since kernel $x:literal) => {
        $crate::paste!(
            #[used]
            #[allow(non_upper_case_globals)]
            #[linkme::distributed_slice(crate::KMI_TABLE)]
            static [< Ke $n >]: $crate::Kexport = $crate::Kexport($($i)::+ as *const (), $crate::parse_version(stringify!($x)), stringify!($($i)::+));
        );
    };
    ($($i:tt)::+ => $n:ident, since $x:literal) => {
        $crate::paste!(
            #[used]
            #[allow(non_upper_case_globals)]
            #[linkme::distributed_slice(crate::KMI_TABLE)]
            static [< Me $n >]: $crate::ImExport = $crate::ImExport($($i)::+ as *const (), $crate::parse_version(stringify!($x)));
        );
    };
    ($($i:tt)::+, since $x:literal) => {
        $crate::paste!(
            #[used]
            #[allow(non_upper_case_globals)]
            #[linkme::distributed_slice(crate::KMI_TABLE)]
            static [< Me $n >]: $crate::ImExport = $crate::ImExport($n as *const (), $crate::parse_version(stringify!($x)));
        );
    };
    ( $(#[$attr:meta])* $vis:vis fn $x:ident as $n:ident ( $($name:ident : $aty:ty),* ) $( -> $rty:ty )? where kernel $v:literal $b:block ) => {
        $crate::paste!(
            #[allow(non_snake_case)] $(#[$attr])*
            $vis fn $x( $( $name : $aty ),* ) $( -> $rty )? { $b }

            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = concat!("Ke", stringify!($n)))]
            #[linkme::distributed_slice(crate::KMI_TABLE)]
            static $n: $crate::Kexport = $crate::Kexport($x as *const (), $crate::parse_version(stringify!($v)), stringify!($n));
        );
    };
    ( $(#[$attr:meta])* $vis:vis fn $x:ident as $n:ident ( $($name:ident : $aty:ty),* ) $( -> $rty:ty )? where $v:literal $b:block ) => {
        $crate::paste!(
            #[allow(non_snake_case)] $(#[$attr])*
            $vis fn $x( $( $name : $aty ),* ) $( -> $rty )? { $b }

            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = concat!("Me", stringify!($n)))]
            #[linkme::distributed_slice(crate::KMI_TABLE)]
            static $n: $crate::ImExport = $crate::ImExport($x as *const (), $crate::parse_version(stringify!($v)));
        );
    };
    ( $(#[$attr:meta])* $vis:vis fn $n:ident ( $($name:ident : $aty:ty),* ) $( -> $rty:ty )? where kernel $x:literal $b:block ) => {
        $crate::paste!(
            #[allow(non_snake_case)] $(#[$attr])*
            fn [< __stub_ $n >]( $( $name : $aty ),* ) $( -> $rty )? { $b }

            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = concat!("Ke", stringify!($n)))]
            #[linkme::distributed_slice(crate::KMI_TABLE)]
            $vis static $n: $crate::Kexport = $crate::Kexport([< __stub_ $n >] as *const (), $crate::parse_version(stringify!($x)), stringify!($n));
        );
    };
    ( $(#[$attr:meta])* $vis:vis fn $n:ident ( $($name:ident : $aty:ty),* ) $( -> $rty:ty )? where $x:literal $b:block ) => {
        $crate::paste!(
            #[allow(non_snake_case)] $(#[$attr])*
            fn [< __stub_ $n >]( $( $name : $aty ),* ) $( -> $rty )? { $b }

            #[used]
            #[allow(non_upper_case_globals)]
            #[unsafe(export_name = concat!("Me", stringify!($n)))]
            #[linkme::distributed_slice(crate::KMI_TABLE)]
            $vis static $n: $crate::ImExport = $crate::ImExport([< __stub_ $n >] as *const (), $crate::parse_version(stringify!($x)));
        );
    };
}
