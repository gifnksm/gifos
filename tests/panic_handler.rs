#![feature(panic_info_message)]
#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::{
    fmt::{self, Write},
    panic::PanicInfo,
};
use gifos::{serial_print, serial_println, QemuExitCode};

const MESSAGE: &str = "Example panic message from panic_handler test";
const PANIC_LINE: u32 = 19; // adjust this when moving the `panic!` call

entry_point!(test_kernel_main);

fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    serial_print!("panic_handler... ");
    panic!(MESSAGE); // must be in line `PANIC_LINE`
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    check_message(info);
    check_location(info);

    serial_println!("[ok]");
    gifos::exit_qemu(QemuExitCode::Success);
    gifos::hlt_loop();
}

fn fail(error: &str) -> ! {
    serial_println!("[failed]");
    serial_println!("{}", error);
    gifos::exit_qemu(QemuExitCode::Failed);
    gifos::hlt_loop();
}

fn check_location(info: &PanicInfo) {
    let location = info.location().unwrap_or_else(|| fail("no location"));
    if location.file() != file!() {
        fail("file name wrong");
    }
    if location.line() != PANIC_LINE {
        fail("file line wrong");
    }
}

fn check_message(info: &PanicInfo) {
    let message = info.message().unwrap_or_else(|| fail("no message"));
    let mut compare_message = CompareMessage { expected: MESSAGE };
    write!(&mut compare_message, "{}", message).unwrap_or_else(|_| fail("write failed"));
    if !compare_message.expected.is_empty() {
        fail("message shorter than expected message");
    }
}

/// Compares a `fmt::Arguments` instance with the `MESSAGE` string
///
/// To use this type, write the `fmt::Arguments` instance to it using the
/// `write` macro. If the message component matches `MESSAGE`, the `expected`
/// field is the empty string.
struct CompareMessage {
    expected: &'static str,
}

impl fmt::Write for CompareMessage {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.expected.starts_with(s) {
            self.expected = &self.expected[s.len()..];
        } else {
            fail("message not equal to expected message");
        }
        Ok(())
    }
}
