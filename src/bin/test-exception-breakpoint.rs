#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use gifos::{exit_qemu, serial_println};

#[cfg(not(test))]
entry_point!(test_kernel_main);

#[cfg(not(test))]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    gifos::interrupts::init_idt();

    x86_64::instructions::interrupts::int3();

    serial_println!("ok");

    unsafe {
        exit_qemu();
    }
    gifos::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("failed");
    serial_println!("{}", info);
    unsafe {
        exit_qemu();
    }
    gifos::hlt_loop();
}
