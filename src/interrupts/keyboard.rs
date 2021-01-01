use crate::print;
use super::{
    KEYBOARD,
    end_of_interrupt,
    InterruptIndex,
};

use x86_64::structures::idt::InterruptStackFrame;

pub fn read_keyboard() -> u8 {
    unsafe { KEYBOARD.read() }
}

pub extern "x86-interrupt" fn interrupt_handler(
    _stack_frame: &mut InterruptStackFrame)
{
    let code = read_keyboard();

    match code {
        0x2..=0x0B => {
            let num = (code - 0x1) % 10;
            print!("{}", num);
        },
        _ => ()
    };

    end_of_interrupt(InterruptIndex::Keyboard);
}
