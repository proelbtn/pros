#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pros::test_runner)]
#![reexport_test_harness_main = "_start_test"]

use pros::{memory::active_level4_table, println};
use x86_64::VirtAddr;

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
    use x86_64::structures::paging::MapperAllSizes;
    println!("Hello World{}", "!");

    pros::init();

    let mut mapper = unsafe { pros::memory::init(boot_info) };
    let mut frame_allocator = unsafe {
        pros::memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map an unused page
    let page = x86_64::structures::paging::Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    pros::memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};


    #[cfg(test)]
    _start_test();

    println!("It did not crash!");

    pros::hlt_loop();
}
