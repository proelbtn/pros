#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pros::test_runner)]
#![reexport_test_harness_main = "_start_main"]

extern crate alloc;

use bootloader::BootInfo;
use core::panic::PanicInfo;

use pros::println;


#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    pros::init(boot_info);
    _start_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pros::test_panic_handler(info);
}

#[test_case]
fn alloc_box() {
    use alloc::boxed::Box;
    let heap_value_1 = Box::new(1);
    let heap_value_2 = Box::new(2);

    assert_eq!(*heap_value_1, 1);
    assert_eq!(*heap_value_2, 2);
}

#[test_case]
fn alloc_string() {
    use alloc::string::String;
    let s = String::from("test");

    assert_eq!(&s, "test");
}

#[test_case]
fn alloc_vec() {
    use alloc::vec::Vec;
    let mut vec = Vec::new();

    for i in 0..1024 {
        vec.push(i);
    }

    assert_eq!(vec.iter().sum::<u64>(), 523_776);
}