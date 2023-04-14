// Bindings specific to Daisogen
#![allow(missing_docs)]

pub mod asm;
mod highlevel;
mod os;
mod pd;
mod servers;

pub use highlevel::*;
pub use os::*;
pub use pd::*;
pub use servers::*;

// Something nice, taken from the kernel
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("debug: {}", format_args!($($arg)*));
    }
}
