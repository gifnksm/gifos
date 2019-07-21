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
    panic!();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("ok");

    unsafe {
        exit_qemu();
    }
    gifos::hlt_loop();
}
