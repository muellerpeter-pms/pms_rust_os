//! Enthält Funktionen für das Drucken von Zeichen auf dem Bildschirm
//!
//! Die Ausführung orientiert sich an den Informationen von [wiki.osdev.org](https://wiki.osdev.org/Printing_to_Screen)

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

/// Farben für den VGA-Modus
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

/// Ein Farbcode, bestehend aus Hintergrund- und Vordergrundfarbe
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
        // > nur für den Editor, der mit der Farbanzeige spinnt, nach der obigen Zeile
    }
}

/// Ein Zeichen, wie es im VGA-Buffer gehalten wird
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

/// Die Puffer-Höhe
const BUFFER_HEIGHT: usize = 25;
/// Die Puffer-Breite
const BUFFER_WIDTH: usize = 80;

/// Der VGA-Puffer als solcher
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Eine Struktur zum Darstellen von Zeichen auf dem Bildschirm
pub struct Writer {
    column_position: usize,
    line_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    fn write_char(&mut self, c: char) {
        const TAB_WIDTH: usize = 4;
        match c {
            '\n' => self.new_line(), // neue Zeile
            '\t' => {
                // Tab
                let tab_pos = self.column_position % TAB_WIDTH; // Position im tab berechnen
                self.column_position += TAB_WIDTH - tab_pos; // Vorrücken zum nächsten tab
            }
            c => {
                if self.column_position >= BUFFER_WIDTH {
                    // Neue Zeile bei Bedarf
                    self.new_line();
                }

                self.buffer.chars[self.line_position][self.column_position].write(ScreenChar {
                    ascii_character: c as u8,
                    color_code: self.color_code,
                });

                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        self.line_position += 1;
        self.column_position = 0;

        if self.line_position >= BUFFER_HEIGHT {
            self.scroll_up();
        }
    }

    fn scroll_up(&mut self) {
        // Alle Zeilen nach oben rücken
        for y in 1..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                self.buffer.chars[y - 1][x].write(self.buffer.chars[y][x].read());
            }
        }

        // letzte Zeile leeren
        let empty = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for x in 0..BUFFER_WIDTH {
            self.buffer.chars[BUFFER_HEIGHT - 1][x].write(empty);
        }

        self.line_position = BUFFER_HEIGHT - 1;
    }

    pub fn clear(&mut self) {
        let empty = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for y in 0..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                self.buffer.chars[y][x].write(empty);
            }
        }
        self.line_position = 0;
        self.column_position = 0;
    }

    pub fn print_string(&mut self, text: &str) {
        for byte in text.bytes() {
            match byte {
                // druckbare Zeichen
                0x20..=0x7e | b'\n' | b'\t' => self.write_char(byte as char),
                // außerhalb der druckbaren Zeichen
                _ => self.write_char(0xfe as char),
            }
        }
    }
}

// Damit können wir auf die implementierten Makros zurück greifen
impl fmt::Write for Writer {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.print_string(text);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        line_position: 0,
        color_code: ColorCode::new(Color::Black, Color::White),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_text::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}


