use core::{cell::UnsafeCell,sync::atomic::{AtomicU64,Ordering},hint};

const WRITER_BIT: u64 = 1 << (u64::BITS - 1);

pub struct KeRwLock<T> {
    state: AtomicU64,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for KeRwLock<T> {}
unsafe impl<T: Send> Sync for KeRwLock<T> {}

impl<T> KeRwLock<T> {
    pub const fn new(t: T) -> Self {
        Self {
            state: AtomicU64::new(0),
            data: UnsafeCell::new(t),
        }
    }

    pub unsafe fn inner(&self) -> &mut T {
        unsafe {
            self.data.get().as_mut_unchecked()
        }
    }

    pub fn read(&self) -> KeRwLockReadGuard<'_, T> {
        loop {
            let old = self.state.load(Ordering::Acquire);
            // If the writer bit is set, spin.
            if old & WRITER_BIT != 0 {
                hint::spin_loop();
                continue;
            }
            // Try to increment the reader count.
            let new = old + 1;
            // The increment must not overflow into the writer bit.
            debug_assert!(new & WRITER_BIT == 0);
            if self
                .state
                .compare_exchange_weak(old, new, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
            {
                return KeRwLockReadGuard { lock: self };
            }
            // CAS failed, retry.
        }
    }

    pub fn try_read(&self) -> Option<KeRwLockReadGuard<'_, T>> {
        let old = self.state.load(Ordering::Acquire);
        if old & WRITER_BIT != 0 {
            return None;
        }
        let new = old + 1;
        if new & WRITER_BIT == 0
            && self
                .state
                .compare_exchange(old, new, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
        {
            Some(KeRwLockReadGuard { lock: self })
        } else {
            None
        }
    }

    pub fn write(&self) -> KeRwLockWriteGuard<'_, T> {
        loop {
            let old = self.state.load(Ordering::Acquire);
            // If the lock is not free, spin.
            if old != 0 {
                hint::spin_loop();
                continue;
            }
            // Try to set the writer bit.
            if self
                .state
                .compare_exchange_weak(0, WRITER_BIT, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
            {
                return KeRwLockWriteGuard { lock: self };
            }
        }
    }

    pub fn try_write(&self) -> Option<KeRwLockWriteGuard<'_, T>> {
        let old = self.state.load(Ordering::Acquire);
        if old != 0 {
            return None;
        }
        if self
            .state
            .compare_exchange(0, WRITER_BIT, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
        {
            Some(KeRwLockWriteGuard { lock: self })
        } else {
            None
        }
    }
}

pub struct KeRwLockReadGuard<'a, T> {
    lock: &'a KeRwLock<T>,
}

impl<T> core::ops::Deref for KeRwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> Drop for KeRwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        // Decrement the reader count with Release ordering to ensure that all
        // reads are visible before any future writes.
        self.lock.state.fetch_sub(1, Ordering::Release);
    }
}

pub struct KeRwLockWriteGuard<'a, T> {
    lock: &'a KeRwLock<T>,
}

impl<T> core::ops::Deref for KeRwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> core::ops::DerefMut for KeRwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for KeRwLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        // Release the write lock with Release ordering.
        self.lock.state.store(0, Ordering::Release);
    }
}
