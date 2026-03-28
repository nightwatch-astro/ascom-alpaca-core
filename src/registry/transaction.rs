use std::sync::atomic::{AtomicU32, Ordering};

/// Thread-safe, monotonically increasing transaction ID generator.
///
/// Starts at 1 (0 is reserved for "no transaction").
pub struct TransactionCounter {
    next: AtomicU32,
}

impl TransactionCounter {
    pub fn new() -> Self {
        Self {
            next: AtomicU32::new(1),
        }
    }

    /// Returns the next transaction ID, incrementing the counter atomically.
    pub fn next(&self) -> u32 {
        self.next.fetch_add(1, Ordering::Relaxed)
    }
}

impl Default for TransactionCounter {
    fn default() -> Self {
        Self::new()
    }
}
