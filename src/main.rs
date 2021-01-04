#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pros::test_runner)]
#![reexport_test_harness_main = "_start_test"]

extern crate alloc;

use pros::println;

use alloc::vec::Vec;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    pros::exit_qemu(pros::QemuExitCode::Failed);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    pros::test_panic_handler(info);
}

bootloader::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static bootloader::BootInfo) -> ! {
    println!("Hello World{}", "!");

    pros::init(boot_info);

    #[cfg(test)]
    _start_test();

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }

    println!("vec at {:p}", vec.as_slice());

    println!("It did not crash!");

    pros::hlt_loop();
}
