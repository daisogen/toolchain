#![allow(unreachable_code)]

// This is a simple Mutex implementation based on Daisogen kernel's implementation
// of futexes.

use super::super::daisogen::futex;
use crate::sync::atomic::{AtomicUsize, Ordering};

pub struct Mutex {
    futex: AtomicUsize,
}

impl Mutex {
    #[inline]
    pub const fn new() -> Mutex {
        Mutex {
            futex: AtomicUsize::new(0),
        }
    }

    #[inline]
    pub fn try_lock(&self) -> bool {
        // Try to change 0 (free) to 1 (locked)
        self.futex
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    pub fn lock(&self) {
        // This while takes care of spurious wakeups, which DO occur in this
        // implementation (and I think all), since there's a race condition
        // below.
        while !self.try_lock() {
            // The lock is held, wait for it to become free (0)
            futex::wait(&self.futex, 1);
        }
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        // Change to 0 (free), and wake another thread
        self.futex.swap(0, Ordering::Release);
        // Between these two lines is the said race condition, can you
        // figure it out?
        futex::wake_one(&self.futex);
    }
}
