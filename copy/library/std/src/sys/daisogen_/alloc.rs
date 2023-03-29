use crate::alloc::{GlobalAlloc, Layout, System};
use crate::ptr;

// These are temporal until I have my neat Mutex
use super::spin_mutex::SpinMutex;
use core::sync::atomic::{AtomicBool, Ordering};

struct Alloc;
static DLMALLOC: SpinMutex<dlmalloc::Dlmalloc<Alloc>> =
    SpinMutex::new(dlmalloc::Dlmalloc::new_with_allocator(Alloc {}));

unsafe impl dlmalloc::Allocator for Alloc {
    // Allocate system resources (not an actual userspace allocation!)
    fn alloc(&self, _size: usize) -> (*mut u8, usize, u32) {
        // This will turn into a Once<> in the future (TODO)
        static INIT: AtomicBool = AtomicBool::new(false);
        // This function is protected by the global lock
        if !INIT.swap(true, Ordering::Relaxed) {
            (
                super::daisogen::heap_base(),
                super::daisogen::heap_size(),
                0,
            )
        } else {
            (ptr::null_mut(), 0, 0)
        }
    }

    fn remap(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize, _can_move: bool) -> *mut u8 {
        ptr::null_mut()
    }

    fn free_part(&self, _ptr: *mut u8, _oldsize: usize, _newsize: usize) -> bool {
        false
    }

    fn free(&self, _ptr: *mut u8, _size: usize) -> bool {
        return false;
    }

    fn can_release_part(&self, _flags: u32) -> bool {
        false
    }

    fn allocates_zeros(&self) -> bool {
        false
    }

    fn page_size(&self) -> usize {
        0x1000
    }
}

// ---

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { DLMALLOC.lock().malloc(layout.size(), layout.align()) }
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { DLMALLOC.lock().calloc(layout.size(), layout.align()) }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            DLMALLOC.lock().free(ptr, layout.size(), layout.align());
        }
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe {
            DLMALLOC
                .lock()
                .realloc(ptr, layout.size(), layout.align(), new_size)
        }
    }
}
