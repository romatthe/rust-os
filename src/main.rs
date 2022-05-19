// We don't want to link the Rust standard library
#![no_std]
// We don't want to enable the standard Rust entry points
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

/// We don't the C language runtime here, so we need to define our own entry point.
/// The linker will assume a function called `_start` as the default entry point 
/// instead (though it is not clear to me why it decides this is the default?).
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Case 0xb8000 to a raw pointer
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This function is called on a panic. It just loops and never terminates.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
