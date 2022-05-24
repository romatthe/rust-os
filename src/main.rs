// We don't want to link the Rust standard library
#![no_std]
// We don't want to enable the standard Rust entry points
#![no_main]
// The default Rust test framework doesn't actually work in `no_std` environments,
// so we'll use our own
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

/// We don't have the C language runtime here, so we need to define our own entry point.
/// The linker will assume a function called `_start` as the default entry point 
/// instead (though it is not clear to me why it decides this is the default?).
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    // We just run our test cases here when our binary is conditionally compiled
    // for test releases
    #[cfg(test)]
    test_main();

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
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Writes an exit code to the special QEMU isa-debug-exit device mounted at IO port `0xf4`.
/// NOTE: See `cargo.toml` test-args for how this device is mounted in QEMU. 
pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }

    // Exit QEMU through the isa-debug-exit device
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    serial_print!("Trivial assertion... ");
    assert_eq!(1, 2);
    serial_println!("[ok]")
}