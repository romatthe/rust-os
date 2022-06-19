use crate::{gdt, println};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt
                .double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

/// Initialize the interrupt descriptor table (IDT). The IDT is used by the processor 
/// to determine the correct response to interrupts and exceptions.
pub fn init_idt() {
    IDT.load();
}

/// Handles the breakpoint exception, which is used in debugger. When the user sets a breakpoint, 
/// the debugger overwrites the corresponding instruction with the `int3` instruction so that the 
/// CPU throws the breakpoint exception when it reaches that line. When the user wants to continue 
/// the program, the debugger replaces the `int3` instruction with the original instruction again 
/// and continues the program.
extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", frame);
}

/// Handles the double-fault exception, which is triggered whenever no appropriate exception handler is
/// registered in the IDT.  
extern "x86-interrupt" fn double_fault_handler(frame: InterruptStackFrame, error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAUL\n{:#?}", frame);
}

#[test_case]
fn test_breakpoint_exception() {
    // Invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}