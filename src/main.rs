#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
mod vga_buffer;
mod serial;
// We use no mange to disable name mangling, so that the output actually,
// has the name _start, as opposed to K#JH$K28294.sd3.core$_fn.2392032

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}
#[no_mangle] 
pub extern "C" fn _start() -> ! {
    println!("Kurogane OS will be with you shortly.");
    println!("Keep tabs at")
    #[cfg(test)]
    test_main();
    // Extern "C" tells the compiler that it should use the C calling convention
    // Casts the hexadecimal integer to a raw pointer
    // raw pointers can ignore borrowing rules, having both mutable and 
    // immutable pointers to the same location
    // Aren't guaranteed to point to valid memory
    // Can be null
    // Don't have automatic cleanup
    // We use enumerate to get a running variable, and we use offset method
    // to write the string and the corresponding color byte.

    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // This function should never return,
    // so we mark it as a diverging function, with the "never" type `!`.
        // Diverging functions are functions that do not return.
        // ! Represents the type of computations which never resolve to a value,
        // such as exit().
        // break, continue and return are of type `!`.
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error {}\n", info);
    exit_qemu(QemuExitCode::Failure);
}
#[test_case]
fn trivial_assertion() {
    serial_print!("Trivial assertion...");
    assert_eq!(1, 1);
    serial_println!("[ok]")
}