#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;
use gifos::println;

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use gifos::interrupts::PICS;

    println!("Hello World! {}", 32 + 3);

    gifos::gdt::init();
    gifos::interrupts::init_idt();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    let ptr = 0xdead_beaf as *mut u32;
    unsafe {
        *ptr = 42;
    }

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
