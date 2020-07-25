use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed.")
}

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = { 
        let mut serial_port = unsafe { SerialPort::new(0x3F8)}; // We wrap this write to a port in unsafe
                                                                // as we can't always guarantee the availability of a given port.
        serial_port.init();
        Mutex::new(serial_port)
    };
}
#[macro_export] // Exporting our print functions for callability from other modules.
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_println!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}