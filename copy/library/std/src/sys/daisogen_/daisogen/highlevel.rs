pub fn yld() {
    super::pd_call0("yld");
}

pub fn ioapic_redirect_irq(irq: u8) -> (u32, u8) {
    let ret = super::pd_call1("ioapic_redirect_irq", irq as u64);
    let gsi = (ret >> 8) as u32;
    let vec = ret as u8;
    (gsi, vec)
}

pub fn unmask(gsi: u32) {
    super::pd_call1("unmask", gsi as u64);
}

pub fn set_vector(v: u8, addr: u64, ist: u32) {
    super::pd_call3("set_vector", v as u64, addr, ist as u64);
}

pub fn eoi() {
    super::pd_call0("eoi");
}

pub fn set_simple_vector(v: u8, addr: u64) {
    super::pd_call2("set_simple_vector", v as u64, addr);
}

pub mod futex {
    use crate::sync::atomic::AtomicUsize;

    pub fn wait(var: &AtomicUsize, val: usize) {
        super::super::pd_call2("futex_wait", var as *const AtomicUsize as u64, val as u64);
    }

    pub fn wake_one(var: &AtomicUsize) {
        super::super::pd_call1("futex_wake_one", var as *const AtomicUsize as u64);
    }
}

pub fn phys_alloc(npages: usize) -> Result<usize, ()> {
    let ptr = super::pd_call1("phys_alloc", npages as u64);
    if ptr == 0 {
        Err(())
    } else {
        Ok(ptr as usize)
    }
}

pub fn phys_of(ptr: usize) -> Result<usize, ()> {
    let ptr = super::pd_call1("phys_of", ptr as u64);
    if ptr == 0 {
        Err(())
    } else {
        Ok(ptr as usize)
    }
}
