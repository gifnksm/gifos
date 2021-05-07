#![feature(custom_test_frameworks)]
#![no_std]
#![no_main]
#![test_runner(gifos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use gifos::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    gifos::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
