use core::{alloc::Layout, cmp::max, sync::atomic::AtomicU16};

#[repr(C)]
struct HdlMeta {
    refc    :   AtomicU16   ,
    size    :   u32         ,
    align   :   u16         ,
    at      :   *mut u8     ,
    #[cfg(debug_assertions)]
    align_t :   u16         ,
    drop    :   fn(*const()),
//  data    :   T           ,
}

pub struct Hdl<const _T: usize> ( *const () /* points to HdlMeta.data */ );

impl<const _T: usize> core::fmt::Debug for Hdl<_T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("Hdl![{:p}{}]", self.0, if self.meta().drop as usize != 0 { " + Drop" } else { "" }))
    }
}

impl<const _T: usize> Clone for Hdl<_T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        let meta = unsafe { ((self.0 as usize - size_of::<HdlMeta>()) as *mut HdlMeta).as_mut_unchecked() };
        meta.refc.fetch_add(1, core::sync::atomic::Ordering::AcqRel);
        Self(self.0)
    }
}

impl<const _T: usize> Drop for Hdl<_T> {
    #[inline(always)]
    fn drop(&mut self) {
        let meta = unsafe { ((self.0 as usize - size_of::<HdlMeta>()) as *mut HdlMeta).as_mut_unchecked() };
        let old = meta.refc.fetch_sub(1, core::sync::atomic::Ordering::AcqRel);

        if old == 1 {
            (meta.drop)(self.0);
            let layout = unsafe { Layout::from_size_align_unchecked(meta.size as usize, meta.align as usize) };
            unsafe { alloc::alloc::dealloc(meta.at, layout) };
        }
    }
}

pub const fn ceil_pow2(x: usize) -> usize {
    if x == 0 { return 0 }
    let mut rv: usize = 1;
    while rv < x { rv <<= 1 }
    rv
}

// internal methods
impl<const _T: usize> Hdl<_T> {
    #[inline(always)]
    fn meta(&self) -> &mut HdlMeta {
        unsafe { ((self.0 as usize - size_of::<HdlMeta>()) as *mut HdlMeta).as_mut_unchecked() }
    }
}

// constructors
impl<const _T: usize> Hdl<_T> {
    pub fn new<T>(t: T) -> Self {
        let align = max(align_of::<T>(), 8);
        let padding = (align - size_of::<HdlMeta>() % align) % align;
        let size = padding + size_of::<T>();
        let layout = unsafe { Layout::from_size_align_unchecked(size, align) };
        let addr = unsafe { alloc::alloc::alloc(layout) };
        let meta = unsafe { &mut core::ptr::read_unaligned((addr as usize + padding) as *mut HdlMeta) };
        let data = unsafe { &mut core::ptr::read_unaligned((addr as usize + padding + size_of::<HdlMeta>()) as *mut T) };
        meta.at = addr;
        meta.align = align as u16;
        meta.drop = |_|();
        #[cfg(debug_assertions)] { meta.align_t = align_of::<T>() as u16; }
        *data = t;
        Self((addr as usize + padding + size_of::<HdlMeta>()) as *mut ())
    }

    pub fn new_drop<T>(t: T, drop: fn(*const())) -> Self {
        let align = max(align_of::<T>(), 8);
        let padding = (align - size_of::<HdlMeta>() % align) % align;
        let size = padding + size_of::<T>();
        let layout = unsafe { Layout::from_size_align_unchecked(size, align) };
        let addr = unsafe { alloc::alloc::alloc(layout) };
        let meta = unsafe { &mut core::ptr::read_unaligned((addr as usize + padding) as *mut HdlMeta) };
        let data = unsafe { &mut core::ptr::read_unaligned((addr as usize + padding + size_of::<HdlMeta>()) as *mut T) };
        meta.at = addr;
        meta.align = align as u16;
        meta.drop = drop;
        #[cfg(debug_assertions)] { meta.align_t = align_of::<T>() as u16; }
        *data = t;
        Self((addr as usize + padding + size_of::<HdlMeta>()) as *mut ())
    }
}

// transformers
impl<const _T: usize> Hdl<_T> {
    #[inline(always)]
    pub fn downcast<T>(&self) -> &T {
        let meta = self.meta();
        #[cfg(debug_assertions)]
        {
            debug_assert!(meta.align_t as usize == align_of::<T>());
            debug_assert!(meta.at as usize - meta.size as usize + self.0 as usize == size_of::<T>());
        };
        unsafe { (self.0 as *const T).as_ref_unchecked() }
    }

    #[inline(always)]
    pub fn downcast_mut<T>(&mut self) -> &mut T {
        let meta = self.meta();
        #[cfg(debug_assertions)]
        {
            debug_assert!(meta.align_t as usize == align_of::<T>());
            debug_assert!(meta.at as usize - meta.size as usize + self.0 as usize == size_of::<T>());
        };
        unsafe { (self.0 as *mut T).as_mut_unchecked() }
    }

    #[inline(always)]
    pub fn to_raw(&self) -> usize { self.0 as _ }

    #[inline(always)]
    pub unsafe fn from_raw(addr: usize) -> Self {
        // NOTE: it's QUITE IMPORTANT to call `.clone()` here as we
        // are trying to create new reference to existing object!
        //
        // If we didn't do this, the object's counter could underflow,
        // as this reference decrements it on drop just like the rest.
        // That ends in either a panic or a memory leak, and we need
        // neither.
        Self(addr as _).clone()
    }
}

// rc_* methods - unified methods (uses `usize` for interface). this allows to abstract internal RC size if needed.
//
// NOTE: Don't use these methods in modules! they are here only for kernel usage.
impl<const _T: usize> Hdl<_T> {
    pub unsafe fn rc_load(&self) -> usize { self.meta().refc.load(core::sync::atomic::Ordering::Relaxed) as _ }
    pub unsafe fn rc_store(&self, v: usize) { self.meta().refc.store(v as _, core::sync::atomic::Ordering::Release) }
    pub unsafe fn rc_add(&self, v: usize) -> usize { self.meta().refc.fetch_add(v as _, core::sync::atomic::Ordering::AcqRel) as _ }
    pub unsafe fn rc_sub(&self, v: usize) -> usize { self.meta().refc.fetch_sub(v as _, core::sync::atomic::Ordering::AcqRel) as _ }
    pub unsafe fn rc_inc(&self) -> usize { unsafe { self.rc_add(1) } }
    pub unsafe fn rc_dec(&self) -> usize { unsafe { self.rc_sub(1) } }
}

pub macro Hdl($($x:tt)+) { Hdl<{ crate::hash!(stringify!($($x)+).as_bytes()) as usize }> }

// #[macro_export] macro_rules! Hdl { ($($x:tt)+) => { Hdl<{ crate::hash!(stringify!($($x)+).as_bytes()) as usize }> } }
