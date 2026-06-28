use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
    arch::asm
};

#[derive(Debug)]
pub struct KeNutex<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for KeNutex<T> {}
unsafe impl<T: Send> Sync for KeNutex<T> {}

impl<T> KeNutex<T> {
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

    pub fn lock(&self) -> KeNutexGuard<'_, T> {
        // Save the current interrupt state before we disable interrupts.
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

        // Spin until we acquire the lock.
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        KeNutexGuard {
            mutex: self,
            saved_if: (rflags & (1 << 9)) != 0,  // Bit 9 is the IF flag in RFLAGS.
        }
    }

    #[allow(dead_code)]
    pub fn try_lock(&self) -> Option<KeNutexGuard<'_, T>> {
        // Save the current interrupt state and disable interrupts.
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

        // Try to acquire the lock with a single CAS attempt.
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(KeNutexGuard {
                mutex: self,
                saved_if: (rflags & (1 << 9)) != 0,
            })
        } else {
            unsafe {
                if (rflags & (1 << 9)) != 0 {
                    asm!("sti", options(nomem, nostack, preserves_flags));
                }
            }
            None
        }
    }
}

pub struct KeNutexGuard<'a, T> {
    mutex: &'a KeNutex<T>,
    saved_if: bool,  // Whether interrupts were enabled before locking.
}

impl<T> core::ops::Deref for KeNutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.mutex.data.get().as_ref_unchecked() }
    }
}

impl<T> core::ops::DerefMut for KeNutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for KeNutexGuard<'_, T> {
    fn drop(&mut self) {
        // Release the lock.
        self.mutex.lock.store(false, Ordering::Release);

        // Restore the interrupt state if it was previously enabled.
        unsafe {
            if self.saved_if {
                asm!("sti", options(nomem, nostack, preserves_flags));
            }
        }
    }
}
