#![feature(custom_test_frameworks)]
#![no_std]
#![no_main]
#![test_runner(gifos::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![deny(unsafe_op_in_unsafe_fn)]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use gifos::println;

entry_point!(test_kernel_main);

fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    test_main();

    gifos::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    gifos::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
