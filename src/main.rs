#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(gifos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use gifos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    gifos::init();

    #[cfg(test)]
    test_main();

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

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    gifos::test_panic_handler(info)
}
