#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

/// We don't have the C language runtime here, so we need to define our own entry point.
/// The linker will assume a function called `_start` as the default entry point 
/// instead and then calls the main test function.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

/// This function is called on a panic. It just loops and never terminates.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}