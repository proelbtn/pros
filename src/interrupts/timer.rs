use crate::print;
use super::{
    end_of_interrupt,
    InterruptIndex,
};

use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn interrupt_handler(
    _stack_frame: &mut InterruptStackFrame)
{
    print!(".");
    end_of_interrupt(InterruptIndex::Timer);
}