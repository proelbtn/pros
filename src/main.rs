#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pros::test_runner)]
#![reexport_test_harness_main = "_start_test"]

extern crate alloc;

use pros::println;

// use alloc::boxed::Box;

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
    use x86_64::{PhysAddr, VirtAddr};
    use x86_64::structures::paging::{Page, PhysFrame, PageTableFlags};
    println!("Hello World{}", "!");

    pros::init(boot_info);

    let page = Page::containing_address(VirtAddr::new(0x0));
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    pros::memory::map_to(page, frame, flags).unwrap().flush();

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    #[cfg(test)]
    _start_test();

    println!("It did not crash!");

    pros::hlt_loop();
}
