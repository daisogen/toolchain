pub fn heap_base() -> *mut u8 {
    // Leak an address from the binary, like this function!
    let gb = &heap_base as *const _ as usize;
    let gb = ((gb >> 30) + 1) << 30; // Get the next one (and truncate offset)
    crate::ptr::from_exposed_addr::<u8>(gb) as *mut u8
}

pub fn heap_size() -> usize {
    1 << 29
}
