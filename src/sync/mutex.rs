use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering}
};

pub struct KeMutex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

// Safety: Mutex is Send and Sync if T is Send.
unsafe impl<T: Send> Send for KeMutex<T> {}
unsafe impl<T: Send> Sync for KeMutex<T> {}

impl<T> KeMutex<T> {
    pub const fn new(t: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(t),
        }
    }

    pub unsafe fn inner(&self) -> &mut T {
        unsafe {
            self.data.get().as_mut_unchecked()
        }
    }

    pub fn lock(&self) -> KeMutexGuard<'_, T> {
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        KeMutexGuard { mutex: self }
    }

    pub fn try_lock(&self) -> Option<KeMutexGuard<'_, T>> {
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(KeMutexGuard { mutex: self })
        } else {
            None
        }
    }
}

pub struct KeMutexGuard<'a, T> {
    mutex: &'a KeMutex<T>,
}

impl<T> core::ops::Deref for KeMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> core::ops::DerefMut for KeMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for KeMutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.lock.store(false, Ordering::Release);
    }
}
