use volatile::Volatile; // Allows us to mark thing as volatile
                        // and make them safe from compiler optimizations
                        // that may exclude non-deterministic results (side effects)
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // Allows comparison for screen characters to test printability.
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    // Returns a full color byte, containing foreground and background.
    fn new(foreground: Color, background: Color) -> ColorCode {
        // ColorCode((background as u8) << 4 | (foreground as u8))
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    // Writes to the last line of a given row, shifting lines up upon
    // completion, pulling in foreground and backgrounds from the ColorCode
    // type.
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        // Method to modify characters VGA buffer, and write single
        // ascii bytes.
        match byte {
            b'\n' => self.new_line(), // return newline at end of row.
            byte => { // if we have a byte of text,
                if self.column_position >= BUFFER_WIDTH { // we want to start it on a newline, if the column position
                                                        // is greater than the length of a buffer row
                    self.new_line();
                }
                
                let row = BUFFER_HEIGHT - 1; // We want to select the current row, as it is somewhat empty, and available for bytes.
                let col = self.column_position; // We select the current column position, as we want to write our char to the immediately available slot.
                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.column_position += 1
            }
        }
    }
    
    pub fn write_string(&mut self, s: &str) {
        // Allows us to print strings to the VGA buffer,
        // by converting them into a sequence of bytes, that we
        // print one by one.
        for byte in s.bytes() {
            match byte {
                // covers ascii byte range via hexadecimal.
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe) // Rust strings are utf-8, so we have to catch any characters
                                            // that are outside of the range VGA can display
            }
        }
    }
    fn new_line(&mut self) {
        // Function to reset cursor position to the beginning of the
        // next available row in our buffer.
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Magenta, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer)}
    };
    writer.write_byte(b'H');
    writer.write_string("IF A DOG CHEWS SHOES WHOSE SHOES DOES HE CHOOSE?");
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}

lazy_static! {
    // Delays initialization of a static value until it is
    // referenced, allowing us to do more set up in the initialization,
    // and read run-time values.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        // We use a mutex here for interior mutability, so that
        // we want mutate our writers contents, without having to
        // make the entire structure mutable.
        // This mutex initiates a spinlock (which doesn't depend on blocking threads)
        // which initiates a tight loop of continuous locking to deny access
        // until the mutex is free again.
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
    });
}


// Macros are defined by rules, one for calls without args,
// and additional rules for expanding and evaluating calls with args.
#[macro_export] // exports module, makes it available from "root" level.
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(||{
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple_output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test-Println_many output")
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(||{
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    })
}
