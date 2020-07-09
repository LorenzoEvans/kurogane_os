#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]
use kurogane_os::serial_print;
use kurogane_os::{exit_qemu, QemuExitCode, serial_println};
use x86_64::structures::idt::InterruptStackFrame;
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: &mut InterruptStackFrame,
    _error_code: u64
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
lazy_static! {
    // A static item is similar to a constant, but it represents a precise
    // location in memory, and has a lifetime that outlives all other lifetimes.
    // ref is syntactic sugar over `&`
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            // Unsafe allows us to:
                // Dereference a raw pointer
                // Call an unsafe function or method
                // Access or modify a mutable static variable
                // Implement an unsafe trait
                // Access fields of unions
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(kurogane_os::gdt::DOUBLE_FAULT_IST_INDEX);
            // 
        }
        idt
    };
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    kurogane_os::gdt::init();
    init_test_idt();

    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kurogane_os::test_panic_handler(info)
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read();
    // This forbids the compiler from implementing TCO recursion
        // TCO is an optimization that allows a compiler to unbundle
        // a function whose last statement is a recursive call
        // into a normal loop, meaning no stack frame is added.
}

pub fn init_test_idt() {
    TEST_IDT.load();
}