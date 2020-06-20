#![feature(exclusive_range_pattern)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;

// We use no mange to disable name mangling, so that the output actually,
// has the name _start, as opposed to K#JH$K28294.sd3.core$_fn.2392032

static HELLO: &[u8] = b"Enter Kurogane OS: Save yourself- everything else? Get a thumb drive.";
#[no_mangle] 
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
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
