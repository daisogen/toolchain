const HH: usize = 0xFFFF800000000000;
pub fn pidof(addr: usize) -> usize {
    (addr - HH) >> 31
}

extern "C" {
    #[link_name = "llvm.returnaddress"]
    pub fn return_address(a: i32) -> *const u8;
}

#[macro_export]
macro_rules! alloc_caller {
    ($size:expr, $align:expr) => {
        $crate::daisogen::pd_call2(
            &format!(
                "alloc_{}",
                $crate::daisogen::pidof($crate::daisogen::return_address(0) as usize)
            ),
            $size,
            $align,
        )
    };
}

#[inline(always)]
pub fn alloc_caller_serialized<T: bincode::Encode>(e: &T) -> usize {
    let config = bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();
    let serialized = bincode::encode_to_vec(e, config).unwrap();
    let sz = 8 + serialized.len();
    let ret = unsafe { alloc_caller!(sz as u64, 8) } as usize;
    let slice = crate::ptr::from_exposed_addr::<u8>(ret) as *mut u8;
    let slice = unsafe { crate::slice::from_raw_parts_mut(slice, sz) };
    slice[..8].clone_from_slice(&sz.to_le_bytes());
    slice[8..].clone_from_slice(&serialized[..]);
    ret
}

pub fn deserialize<T: bincode::Decode>(ptr: usize) -> T {
    let sz = crate::ptr::from_exposed_addr::<u64>(ptr);
    let sz = unsafe { *sz } as usize;
    let slice = crate::ptr::from_exposed_addr::<u8>(ptr + 8);
    let slice = unsafe { crate::slice::from_raw_parts(slice, sz) };
    let config = bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();
    bincode::decode_from_slice(slice, config).unwrap().0
}
