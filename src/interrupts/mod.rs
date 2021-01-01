pub mod keyboard;
pub mod timer;

use crate::println;
use crate::gdt::DOUBLE_FAULT_IST_INDEX;

use lazy_static::lazy_static;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

const PIC1_OFFSET: u8 = 0x20;
const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

const PIC_EOI: u8 = 0x20;

static mut PIC1_COMMAND: Port<u8> = Port::new(0x20);
static mut PIC1_DATA: Port<u8> = Port::new(0x21);
static mut KEYBOARD: Port<u8> = Port::new(0x60);
static mut PIC2_COMMAND: Port<u8> = Port::new(0xA0);
static mut PIC2_DATA: Port<u8> = Port::new(0xA1);

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.into()]
            .set_handler_fn(timer::interrupt_handler);
        idt[InterruptIndex::Keyboard.into()]
            .set_handler_fn(keyboard::interrupt_handler);
        idt
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC1_OFFSET,
    Keyboard = PIC1_OFFSET + 1,
}

impl From<InterruptIndex> for usize {
    fn from(idx: InterruptIndex) -> Self {
        usize::from(idx as u8)
    }
}

pub fn pic_remap() {
    let mask1 = unsafe { PIC1_DATA.read() };
    let mask2 = unsafe { PIC2_DATA.read() };

    unsafe {
        PIC1_COMMAND.write(0x11);
        PIC2_COMMAND.write(0x11);
        PIC1_DATA.write(PIC1_OFFSET);
        PIC2_DATA.write(PIC2_OFFSET);
        PIC1_DATA.write(4);
        PIC2_DATA.write(2);
        PIC1_DATA.write(0x1);
        PIC2_DATA.write(0x1);
        PIC1_DATA.write(mask1);
        PIC2_DATA.write(mask2);
    }
}

pub fn end_of_interrupt(idx: InterruptIndex) {
    if (idx as u8) >= PIC2_OFFSET {
        unsafe { PIC2_COMMAND.write(PIC_EOI) };
    }
    unsafe { PIC1_COMMAND.write(PIC_EOI) };
}

pub fn init_idt() {
    IDT.load();
}

pub fn init() {
    pic_remap();
    init_idt();
    x86_64::instructions::interrupts::enable();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

