use volatile::Volatile;
use core::fmt::Write;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code, non_camel_case_types)]
pub enum Colours {
    BLACK = 0,
    BLUE = 1,
    GREEN = 2,
    RED = 4,
    LIGHT_GRAY = 7,
    DARK_GRAY = 8,
    PINK = 13,
    YELLOW = 14,
    WHITE = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct Colour(u8);

impl Colour {
    pub fn new(fg: Colours, bg: Colours) -> Self {
        Colour((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    colour: Colour,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    col_pos: usize,
    colour: Colour,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.col_pos;

                let colour = self.colour;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    colour,
                });
                self.col_pos += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // Valid ascii (printable)
                0x20..=0x7e | b'\n' => self.write_byte(byte),

                // Invalid ascii (not printable)
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
        self.col_pos = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank_char = ScreenChar {
            ascii_char: b' ',
            colour: self.colour
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank_char);
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub fn print_something() {
    let mut writer_green = Writer {
        col_pos: 0,
        colour: Colour::new(Colours::GREEN, Colours::BLACK),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    let mut writer_red = Writer {
        col_pos: 0,
        colour: Colour::new(Colours::RED, Colours::BLACK),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    let mut writer_blue = Writer {
        col_pos: 0,
        colour: Colour::new(Colours::BLUE, Colours::BLACK),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    write!(writer_green, "this is green {}\n", 1+1).unwrap();
    write!(writer_red, "this is red {}\n", 1+1).unwrap();
    write!(writer_blue, "this is blue {}\n", 1+1).unwrap();
}
