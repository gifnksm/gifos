#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use gifos::{exit_qemu, serial_print, serial_println, QemuExitCode};

entry_point!(test_kernel_main);

fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    gifos::hlt_loop();
}

fn should_fail() {
    serial_print!("should_fail... ");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    gifos::hlt_loop();
}
