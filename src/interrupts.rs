use crate::{gdt, print, println};
use lazy_static::lazy_static;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::{instructions::port::Port, structures::idt::{InterruptDescriptorTable, InterruptStackFrame}};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(
    unsafe { 
        ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) 
    }
);

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe {
            idt
                .double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

/// Initialize the interrupt descriptor table (IDT). The IDT is used by the processor 
/// to determine the correct response to interrupts and exceptions.
pub fn init_idt() {
    IDT.load();
}

/// Initialize bopth the primary and secondary Programmable Interrupt Controller.
pub fn init_pics() {
    unsafe {
        PICS.lock().initialize();
    }
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
extern "x86-interrupt" fn double_fault_handler(frame: InterruptStackFrame, _: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", frame);
}

/// Handles a hardware interrupt from the Timer slot of the primary PIC.
extern "x86-interrupt" fn timer_interrupt_handler(_: InterruptStackFrame) {
    print!(".");
    
    // Notify the PIC that we're done reacting to the interrupt with an EOI signal
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

/// Handles input interrupts from a PS/2 keyboard.
extern "x86-interrupt" fn keyboard_interrupt_handler(_: InterruptStackFrame) {
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore));
    }

    // Read the keyboard character scancode from IO port 0x60
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    } 

    // Notify the PIC that we're done reacting to the interrupt with an EOI signal
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

#[test_case]
fn test_breakpoint_exception() {
    // Invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}