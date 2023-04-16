use crate::arch::asm;

pub fn in8(port: u16) -> u8 {
    let ret: u8;
    unsafe {
        asm!("in al, dx",
             in("dx") port,
             out("al") ret,
             options(nostack, preserves_flags));
    }
    ret
}

pub fn in16(port: u16) -> u16 {
    let ret: u16;
    unsafe {
        asm!("in ax, dx",
             in("dx") port,
             out("ax") ret,
             options(nostack, preserves_flags));
    }
    ret
}

pub fn in32(port: u16) -> u32 {
    let ret: u32;
    unsafe {
        asm!("in eax, dx",
             in("dx") port,
             out("eax") ret,
             options(nostack, preserves_flags));
    }
    ret
}

// ---

pub fn out8(port: u16, val: u8) {
    unsafe {
        asm!("out dx, al",
             in("dx") port,
             in("al") val,
             options(nostack, preserves_flags));
    }
}

pub fn out16(port: u16, val: u16) {
    unsafe {
        asm!("out dx, ax",
             in("dx") port,
             in("ax") val,
             options(nostack, preserves_flags));
    }
}

pub fn out32(port: u16, val: u32) {
    unsafe {
        asm!("out dx, eax",
             in("dx") port,
             in("eax") val,
             options(nostack, preserves_flags));
    }
}
