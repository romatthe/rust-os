// We don't want to link the Rust standard library
#![no_std]
// We don't want to enable the standard Rust entry points
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

/// We don't have the C language runtime here, so we need to define our own entry point.
/// The linker will assume a function called `_start` as the default entry point 
/// instead (though it is not clear to me why it decides this is the default?).
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("An extremely bad panic");

    loop {}
}

/// This function is called on a panic. It just loops and never terminates.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
