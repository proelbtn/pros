#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pros::test_runner)]
#![reexport_test_harness_main = "_start_test"]

use pros::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    pros::exit_qemu(pros::QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    pros::test_panic_handler(info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    _start_test();

    loop {}
}
