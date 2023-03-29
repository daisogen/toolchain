use super::daisogen;
use crate::sync::atomic::{AtomicU64, Ordering};

pub type Key = usize;

// Since regular pd functions cache addresses using thread_local!(), these must
// handle their cache separately.

// TODO: Make these Once<> when that's available

static CREATE: AtomicU64 = AtomicU64::new(0);
pub unsafe fn create(_dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    if CREATE.load(Ordering::SeqCst) == 0 {
        CREATE.store(daisogen::pd_get_nocache("tls_create"), Ordering::SeqCst);
    }

    unsafe { jmp0(CREATE.load(Ordering::SeqCst)) as Key }
}

static SET: AtomicU64 = AtomicU64::new(0);
pub unsafe fn set(key: Key, value: *mut u8) {
    if SET.load(Ordering::SeqCst) == 0 {
        SET.store(daisogen::pd_get_nocache("tls_set"), Ordering::SeqCst);
    }

    unsafe {
        jmp2(key as u64, value as u64, SET.load(Ordering::SeqCst));
    }
}

static GET: AtomicU64 = AtomicU64::new(0);
pub unsafe fn get(key: Key) -> *mut u8 {
    if GET.load(Ordering::SeqCst) == 0 {
        GET.store(daisogen::pd_get_nocache("tls_get"), Ordering::SeqCst);
    }

    let ret = unsafe { jmp1(key as u64, GET.load(Ordering::SeqCst)) };
    crate::ptr::from_exposed_addr::<u8>(ret as usize) as *mut u8
}

static DESTROY: AtomicU64 = AtomicU64::new(0);
pub unsafe fn destroy(key: Key) {
    if DESTROY.load(Ordering::SeqCst) == 0 {
        DESTROY.store(daisogen::pd_get_nocache("tls_destroy"), Ordering::SeqCst);
    }

    unsafe {
        jmp1(key as u64, DESTROY.load(Ordering::SeqCst));
    }
}

// ---

extern "C" {
    fn jmp0(ptr: u64) -> u64;
    fn jmp1(arg1: u64, ptr: u64) -> u64;
    fn jmp2(arg1: u64, arg2: u64, ptr: u64) -> u64;
}

crate::arch::global_asm!(
    "
jmp0: jmp rdi
jmp1: jmp rsi
jmp2: jmp rdx
"
);
