use core::sync::atomic::{AtomicBool, Ordering};
use core::hint;

pub struct KeBarrier {
    open: AtomicBool,
}

impl KeBarrier {
    pub const fn new() -> Self {
        Self {
            open: AtomicBool::new(false),
        }
    }

    pub fn open(&self) {
        self.open.store(true, Ordering::Release);
    }

    pub fn is_open(&self) -> bool {
        self.open.load(Ordering::Acquire)
    }

    pub fn wait(&self) {
        while !self.is_open() {
            hint::spin_loop();
        }
    }
}
