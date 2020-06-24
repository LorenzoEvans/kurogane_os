#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kurogane_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kurogane_os::println;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
    // by testing this in a basic_boot environment
    // we can ensure our ability to fix our os in the future
    // by making sure panic messages will print without
    // the initialization from calling _start
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kurogane_os::test_panic_handler(info)
}