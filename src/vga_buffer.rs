use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[macro_export]
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

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[allow(dead_code)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
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
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
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

#[cfg(test)]
mod test {
    use super::*;

    fn construct_writer() -> Writer {
        #[cfg(not(test))]
        use std::boxed::Box;

        let buffer = construct_buffer();
        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Blue, Color::Magenta),
            buffer: Box::leak(Box::new(buffer)),
        }
    }

    fn construct_buffer() -> Buffer {
        use array_init::array_init;
        Buffer {
            chars: array_init(|_| array_init(|_| Volatile::new(empty_char()))),
        }
    }

    fn empty_char() -> ScreenChar {
        ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Green, Color::Brown),
        }
    }

    #[test]
    fn write_byte() {
        let mut writer = construct_writer();
        writer.write_byte(b'X');
        writer.write_byte(b'Y');

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == BUFFER_HEIGHT - 1 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'X');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 1 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'Y');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }

    #[test]
    fn write_formatted() {
        use core::fmt::Write;

        let mut writer = construct_writer();
        writeln!(&mut writer, "a").unwrap();
        writeln!(&mut writer, "bc {}", 32 + 3).unwrap();

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == BUFFER_HEIGHT - 3 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'a');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'b');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'c');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 2 {
                    assert_eq!(screen_char.ascii_character, b' ');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 3 {
                    assert_eq!(screen_char.ascii_character, b'3');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i == BUFFER_HEIGHT - 2 && j == 4 {
                    assert_eq!(screen_char.ascii_character, b'5');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else if i >= BUFFER_HEIGHT - 2 {
                    assert_eq!(screen_char.ascii_character, b' ');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }

    #[test]

    fn change_writer_color() {
        use core::fmt::Write;

        let mut writer = construct_writer();
        let color_code1 = writer.color_code;
        let color_code2 = ColorCode::new(Color::Black, Color::White);
        let color_code3 = ColorCode::new(Color::Yellow, Color::Green);
        writeln!(&mut writer, "a").unwrap();
        writer.color_code = color_code2;
        writeln!(&mut writer, "b").unwrap();
        writer.color_code = color_code3;
        writeln!(&mut writer, "c").unwrap();

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == BUFFER_HEIGHT - 4 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'a');
                    assert_eq!(screen_char.color_code, color_code1);
                } else if i == BUFFER_HEIGHT - 3 {
                    if j == 0 {
                        assert_eq!(screen_char.ascii_character, b'b');
                        assert_eq!(screen_char.color_code, color_code2);
                    } else {
                        assert_eq!(screen_char.ascii_character, b' ');
                        assert_eq!(screen_char.color_code, color_code1);
                    }
                } else if i == BUFFER_HEIGHT - 2 {
                    if j == 0 {
                        assert_eq!(screen_char.ascii_character, b'c');
                        assert_eq!(screen_char.color_code, color_code3);
                    } else {
                        assert_eq!(screen_char.ascii_character, b' ');
                        assert_eq!(screen_char.color_code, color_code2);
                    }
                } else if i == BUFFER_HEIGHT - 1 {
                    assert_eq!(screen_char.ascii_character, b' ');
                    assert_eq!(screen_char.color_code, color_code3);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }

    #[test]
    fn shift_lines() {
        use core::fmt::Write;

        let offset = 5;
        let mut writer = construct_writer();
        for i in 0..(BUFFER_HEIGHT + offset) {
            writeln!(writer, "{:>02}", i).unwrap();
        }

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let num = i + offset + 1;
                let screen_char = screen_char.read();
                assert_eq!(screen_char.color_code, writer.color_code);
                if i < BUFFER_HEIGHT - 1 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'0' + (num / 10) as u8);
                } else if i < BUFFER_HEIGHT - 1 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'0' + (num % 10) as u8);
                } else {
                    assert_eq!(screen_char.ascii_character, b' ');
                }
            }
        }
    }

    #[test]
    fn unicode_char() {
        use core::fmt::Write;

        let mut writer = construct_writer();
        write!(&mut writer, "Hello, 世界").unwrap();
        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i < BUFFER_HEIGHT - 1 || j > 12 {
                    assert_eq!(screen_char, empty_char());
                } else {
                    assert_eq!(screen_char.color_code, writer.color_code);
                    let expected = match j {
                        0 => b'H',
                        1 => b'e',
                        2 | 3 => b'l',
                        4 => b'o',
                        5 => b',',
                        6 => b' ',
                        7 | 8 | 9 | 10 | 11 | 12 => 0xfe,
                        _ => panic!(),
                    };
                    assert_eq!(screen_char.ascii_character, expected);
                }
            }
        }
    }
}
