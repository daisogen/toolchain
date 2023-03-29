#![deny(unsafe_op_in_unsafe_fn)]

// This 👏 mutex 👏 is 👏 temporal 👏
#[path = "../sgx/waitqueue/spin_mutex.rs"]
mod spin_mutex; // Stealing sources like a boss

// ---

pub mod alloc;
pub mod args;
#[path = "../unix/cmath.rs"]
pub mod cmath;
#[unstable(feature = "daisogen_api", issue = "none")]
pub mod daisogen;
pub mod env;
pub mod fs;
#[path = "../unsupported/io.rs"]
pub mod io;
pub mod locks;
#[path = "../unsupported/net.rs"]
pub mod net;
#[path = "../unsupported/once.rs"]
pub mod once;
pub mod os;
#[path = "../unix/os_str.rs"]
pub mod os_str;
#[path = "../unix/path.rs"]
pub mod path;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
pub mod stdio;
pub mod thread;
#[cfg(target_thread_local)]
#[path = "../unsupported/thread_local_dtor.rs"]
pub mod thread_local_dtor;
#[path = "../unsupported/thread_local_key.rs"]
pub mod thread_local_key;
#[path = "../unsupported/time.rs"]
pub mod time;

mod common;
pub use common::*;

// Overriden, temporally (maybe forever)
#[panic_handler]
fn panic(info: &crate::panic::PanicInfo<'_>) -> ! {
    let text = format!("Userspace panic: {}", info);
    let strptr = text.as_bytes().as_ptr() as u64;
    let sz = text.as_bytes().len() as u64;
    daisogen::pd_call2_nocache("print", strptr, sz);

    // exit() here and such
    loop {}
}
