use super::super::daisogen::futex;
use super::Mutex;
use crate::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use crate::time::Duration;

pub struct Condvar {
    futex: AtomicUsize,
}

impl Condvar {
    #[inline]
    #[rustc_const_stable(feature = "const_locks", since = "1.63.0")]
    pub const fn new() -> Condvar {
        Condvar {
            futex: AtomicUsize::new(0),
        }
    }

    #[inline]
    pub fn notify_one(&self) {
        self.futex.fetch_add(1, Relaxed);
        futex::wake_one(&self.futex);
    }

    #[inline]
    pub fn notify_all(&self) {
        panic!("notify_all not implemented");
    }

    pub unsafe fn wait(&self, mutex: &Mutex) {
        // Examine notification counter before we unlock de mutex
        let val = self.futex.load(Relaxed);
        // Unlock before going to sleep
        unsafe {
            mutex.unlock();
        }
        // Sleep
        futex::wait(&self.futex, val);
        // Lock again
        mutex.lock();
    }

    pub unsafe fn wait_timeout(&self, _mutex: &Mutex, _dur: Duration) -> bool {
        panic!("condvar wait with duration not supported");
    }
}
