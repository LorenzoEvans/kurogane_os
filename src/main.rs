#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kurogane_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
mod vga_buffer;
mod serial;
// We use no mange to disable name mangling, so that the output actually,
// has the name _start, as opposed to K#JH$K28294.sd3.core$_fn.2392032

#[no_mangle] 
pub extern "C" fn _start() -> ! {

    println!("Kurogane!");
    
    kurogane_os::init();
    
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    }
    
    #[cfg(test)]
    test_main();
    
    println!("In the meantime, save yourself. Everything else? Get a thumb drive.");
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
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
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
    kurogane_os::test_panic_handler(info)
}
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}