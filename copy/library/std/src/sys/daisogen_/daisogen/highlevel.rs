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

pub type FutexID = u64;
pub fn futex_new(var: &usize) -> FutexID {
    super::pd_call1("futex_new", var as *const usize as u64)
}

pub fn futex_wait(id: FutexID, val: usize) {
    super::pd_call2("futex_wait", id as u64, val as u64);
}

pub fn futex_wake_one(id: FutexID) {
    super::pd_call1("futex_wake_one", id as u64);
}
