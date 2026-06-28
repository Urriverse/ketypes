bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct KeEntryFlags: u64 {
        /// The page is present in memory.
        const PRESENT         = 1 <<  0;
        /// The page is writable (for kernel mode, or user if `USER_ACCESSIBLE`).
        const WRITABLE        = 1 <<  1;
        /// The page is accessible from user mode (CPL 3).
        const USER_ACCESSIBLE = 1 <<  2;
        /// Write‑through caching (vs. write‑back).
        const WRITE_THROUGH   = 1 <<  3;
        /// Cache disabled for this page.
        const CACHE_DISABLE   = 1 <<  4;
        /// The page has been accessed (set by hardware).
        const ACCESSED        = 1 <<  5;
        /// The page has been written to (set by hardware).
        const DIRTY           = 1 <<  6;
        /// The entry points to a huge page (2 MiB or 1 GiB).
        const HUGE_PAGE       = 1 <<  7;
        /// The page is global (not flushed on CR3 switch).
        const GLOBAL          = 1 <<  8;
        /// Execute disable (NX bit) – the page cannot be executed.
        const NO_EXECUTE      = 1 << 63;

        // Kernel‑specific software‑managed flags (stored in available bits).
        /// Copy‑on‑write flag (used by the scheduler).
        const COPY_ON_WRITE   = 1 << 52;
        /// File‑mapped flag (for mmap).
        const FILE_MAPPED     = 1 << 53;
        /// Swapped flag (page is swapped out).
        const SWAPPED         = 1 << 54;
    }
}
