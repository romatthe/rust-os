// We don't want to link the Rust standard library
#![no_std]
// We don't want to enable the standard Rust entry points
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

/// We don't have the C language runtime here, so we need to define our own entry point.
/// The linker will assume a function called `_start` as the default entry point 
/// instead (though it is not clear to me why it decides this is the default?).
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello Kernel{}", "!");

    // General initialization routine
    rust_os::init();

    // Invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();

    // We just run our test cases here when our binary is conditionally compiled
    // for test releases
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    
    loop {}
}

/// This function is called on a panic. It just loops and never terminates.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}