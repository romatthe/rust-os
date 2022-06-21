#![no_std]
#![cfg_attr(test, no_main)]
// The default Rust test framework doesn't actually work in `no_std` environments,
// so we'll use our own
#![feature(custom_test_frameworks)]
// Allows the use of the x86-interrupt ABI calling convention
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}

/// Initializastion routine that can be used in the kernel and integration tests
pub fn init() {
    gdt::init();
    interrupts::init_idt();
    interrupts::init_pics();
    x86_64::instructions::interrupts::enable();
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

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where 
    T: Fn() 
{
    fn run(&self) -> () {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    // Exit QEMU through the isa-debug-exit device
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    
    // Exit QEMU through the isa-debug-exit device
    exit_qemu(QemuExitCode::Failed);

    loop {}
}