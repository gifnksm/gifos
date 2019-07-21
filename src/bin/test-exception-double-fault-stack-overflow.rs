#![feature(abi_x86_interrupt)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use gifos::{exit_qemu, gdt, serial_println};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[cfg(not(test))]
entry_point!(test_kernel_main);

#[cfg(not(test))]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    gifos::gdt::init();
    init_test_idt();

    #[allow(unconditional_recursion)]
    fn stack_overflow() {
        stack_overflow();
    }
    stack_overflow();

    serial_println!("failed");
    serial_println!("No exception occured");

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

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

#[cfg(not(test))]
fn init_test_idt() {
    IDT.load();
}

extern "x86-interrupt" fn double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64,
) {
    serial_println!("ok");
    unsafe {
        exit_qemu();
    }
    gifos::hlt_loop();
}
