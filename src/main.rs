#![no_std] // don't link Rust standard library
#![no_main] // disable all Rust level entry points

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // don't mangle function name
pub extern "C" fn _start() -> ! {
    // the linker will look for a function named `_start`, so this function is the entry point
    // the default name is `_start`
    loop {}
}

// this function will be called when panic
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
