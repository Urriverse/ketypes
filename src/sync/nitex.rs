use core::{
    cell::UnsafeCell,
    arch::asm
};

pub struct KeNitex<T> {
    data: UnsafeCell<T>,
}

impl<T: Clone> Clone for KeNitex<T> {
    fn clone(&self) -> Self {
        Self::new((unsafe { &*self.data.get() }).clone())
    }
}

// Safety: Nitex is Send and Sync if T is Send, but only if the caller ensures
// that the lock is only used on a single CPU.
unsafe impl<T: Send> Send for KeNitex<T> {}
unsafe impl<T: Send> Sync for KeNitex<T> {}

impl<T> KeNitex<T> {
    pub const fn new(t: T) -> Self {
        Self { data: UnsafeCell::new(t) }
    }

    pub unsafe fn inner(&self) -> &mut T {
        unsafe {
            self.data.get().as_mut_unchecked()
        }
    }

    pub fn lock(&self) -> KeNitexGuard<'_, T> {
        let rflags: u64;
        unsafe {
            asm!(
                "pushfq",
                "pop {0}",
                out(reg) rflags,
                options(nomem, preserves_flags)
            );
            asm!("cli", options(nomem, nostack, preserves_flags));
        }

        KeNitexGuard { mutex: self, saved_if: (rflags & (1 << 9)) != 0 }
    }
}

pub struct KeNitexGuard<'a, T> {
    mutex: &'a KeNitex<T>,
    saved_if: bool,
}

impl<T> core::ops::Deref for KeNitexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.mutex.data.get().as_ref_unchecked() }
    }
}

impl<T> core::ops::DerefMut for KeNitexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for KeNitexGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            if self.saved_if {
                asm!("sti", options(nomem, nostack, preserves_flags));
            }
        }
    }
}
