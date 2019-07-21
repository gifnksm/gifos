#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use gifos::println;

#[cfg(not(test))]
entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use gifos::{
        interrupts::PICS,
        memory::{self, BootInfoFrameAllocator},
    };
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World! {}", 32 + 3);

    gifos::gdt::init();
    gifos::interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    let mut mapper = unsafe { memory::init(boot_info.physical_memory_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map a previously unmapped page
    let page = Page::containing_address(VirtAddr::new(0x1000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    println!("It did not crash!");
    gifos::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    gifos::hlt_loop();
}
