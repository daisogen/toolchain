use crate::arch::global_asm;
use crate::cell::RefCell;
use crate::collections::HashMap;
use crate::string::String;

// --- HIGH LEVEL ---

// Since this is a cache, it's meant to be fast, so let's make it thread_local
// in order to avoid mutex overhead
thread_local! {
    static PD_CACHE: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
}

pub fn pd_get_nocache(name: &str) -> u64 {
    let strptr = name.as_bytes().as_ptr() as u64;
    let sz = name.as_bytes().len();
    unsafe { jmp_pd_get(strptr, sz) }
}

pub fn pd_get(name: &str) -> u64 {
    let cached = PD_CACHE.with(|c| match c.borrow().get(name) {
        Some(x) => Some(*x),
        None => None,
    });
    if let Some(cached) = cached {
        return cached;
    }

    let ret = pd_get_nocache(name);
    PD_CACHE.with(|c| c.borrow_mut().insert(name.to_string(), ret));
    ret
}

pub fn pd_set_nocache(name: &str, val: u64) {
    let strptr = name.as_bytes().as_ptr() as u64;
    let sz = name.as_bytes().len();
    unsafe {
        jmp_pd_set(strptr, sz, val);
    }
}

pub fn pd_set(name: &str, val: u64) {
    pd_set_nocache(name, val);
    PD_CACHE.with(|c| c.borrow_mut().insert(name.to_string(), val));
}

// --- LOW LEVEL ---

// TODO: Please please make this tidier. I tried but the macro seems complex.

pub fn pd_call0(name: &str) -> u64 {
    unsafe { jmp0(pd_get(name)) }
}

pub fn pd_call0_nocache(name: &str) -> u64 {
    unsafe { jmp0(pd_get_nocache(name)) }
}

pub fn pd_call1(name: &str, arg1: u64) -> u64 {
    unsafe { jmp1(arg1, pd_get(name)) }
}

pub fn pd_call1_nocache(name: &str, arg1: u64) -> u64 {
    unsafe { jmp1(arg1, pd_get_nocache(name)) }
}

pub fn pd_call2(name: &str, arg1: u64, arg2: u64) -> u64 {
    unsafe { jmp2(arg1, arg2, pd_get(name)) }
}

pub fn pd_call2_nocache(name: &str, arg1: u64, arg2: u64) -> u64 {
    unsafe { jmp2(arg1, arg2, pd_get_nocache(name)) }
}

pub fn pd_call3(name: &str, arg1: u64, arg2: u64, arg3: u64) -> u64 {
    unsafe { jmp3(arg1, arg2, arg3, pd_get(name)) }
}

pub fn pd_call3_nocache(name: &str, arg1: u64, arg2: u64, arg3: u64) -> u64 {
    unsafe { jmp3(arg1, arg2, arg3, pd_get_nocache(name)) }
}

pub fn pd_call4(name: &str, arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> u64 {
    unsafe { jmp4(arg1, arg2, arg3, arg4, pd_get(name)) }
}

pub fn pd_call4_nocache(name: &str, arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> u64 {
    unsafe { jmp4(arg1, arg2, arg3, arg4, pd_get_nocache(name)) }
}

pub fn pd_call5(name: &str, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) -> u64 {
    unsafe { jmp5(arg1, arg2, arg3, arg4, arg5, pd_get(name)) }
}

pub fn pd_call5_nocache(name: &str, arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64) -> u64 {
    unsafe { jmp5(arg1, arg2, arg3, arg4, arg5, pd_get_nocache(name)) }
}

extern "C" {
    fn jmp_pd_get(strptr: u64, sz: usize) -> u64;
    fn jmp_pd_set(stprtr: u64, sz: usize, val: u64) -> u64;
    fn jmp0(ptr: u64) -> u64;
    fn jmp1(arg1: u64, ptr: u64) -> u64;
    fn jmp2(arg1: u64, arg2: u64, ptr: u64) -> u64;
    fn jmp3(arg1: u64, arg2: u64, arg3: u64, ptr: u64) -> u64;
    fn jmp4(arg1: u64, arg2: u64, arg3: u64, arg4: u64, ptr: u64) -> u64;
    fn jmp5(arg1: u64, arg2: u64, arg3: u64, arg4: u64, arg5: u64, ptr: u64) -> u64;
}

global_asm!(
    "
jmp_pd_get:
    mov rax, 0xFFFF800000000000
    mov rax, qword ptr [rax]
    jmp rax
jmp_pd_set:
    mov rax, 0xFFFF800000000008
    mov rax, qword ptr [rax]
    jmp rax
jmp0: jmp rdi
jmp1: jmp rsi
jmp2: jmp rdx
jmp3: jmp rcx
jmp4: jmp r8
jmp5: jmp r9
"
);
