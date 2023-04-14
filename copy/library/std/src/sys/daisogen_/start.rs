// Entry point calls this function, which would be defined in the libC runtime
// But that's not present in Daisogen, so let's write it up
#[no_mangle]
pub extern "C" fn __libc_start_main() {
    // Prepare public alloc function
    let mypid = super::daisogen::pidof(__libc_start_main as usize);
    let name = format!("alloc_{}", mypid);
    crate::daisogen::pd_set(&name, puballoc as u64);

    // Call main()
    extern "C" {
        fn main();
    }
    unsafe {
        main();
    }

    // TODO: exit()
}

extern "C" fn puballoc(size: usize, align: usize) -> usize {
    let layout = crate::alloc::Layout::from_size_align(size, align).unwrap();
    let ptr = unsafe { crate::alloc::alloc(layout) };
    ptr as usize
}
