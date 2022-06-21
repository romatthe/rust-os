#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use rust_os::{QemuExitCode, exit_qemu, serial_print, serial_println};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(rust_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    rust_os::gdt::init();
    init_test_idt();

    // Trigger a stack overflow
    stack_overflow();

    panic!("Execution continued after stack overflow");
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(_: InterruptStackFrame, _: u64) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);    
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    // For each recursion the return address is pushed on the stack
    stack_overflow();
    // Prevent tail recursion optimization
    volatile::Volatile::new(0).read();
}

