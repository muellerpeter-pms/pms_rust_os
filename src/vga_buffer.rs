//! Handhabung aller Operationen auf dem VGA-Text-Buffer

use volatile::Volatile;
use x86_64::instructions::interrupts;

/// Farben im VGA-Text-Modus.
#[allow(missing_docs)]
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

/// Struktur für den Farbcode aus 4 bit Vordergrund und 4 bit Hintergrund.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    /// Erzeugt einen neuen Farbcode.
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// Enthält die Informationen eines Zeichens auf dem Bildschirm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    /// ASCII Code des Zeichens
    ascii_character: u8,
    /// Farbkodierung
    color_code: ColorCode,
}

/// Höhe des VGA-Text-Buffer
const BUFFER_HEIGHT: usize = 25;
/// Breite des VGA-Text-Buffer
const BUFFER_WIDTH: usize = 80;

/// Struktur, welche den VGA-Text-Buffer repräsentiert.
#[repr(transparent)]
struct Buffer {
    /// Die einzelnen Zeichen, als Array aus [BreitexHöhe]
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Schreibt auf den VGA-Buffer.
pub struct Writer {
    /// Position in der Spalte
    column_position: usize,
    /// Aktueller Farbcode für das Schreiben.
    color_code: ColorCode,
    /// der Puffer, in welchen wir schreiben.
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Schreibt ein einzelnes Byte (Zeichen) in den Puffer
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

    /// Erzeugt eine neue Zeile im Puffer
    ///
    /// Dabei werden alle Zeilen jeweils eins nach oben kopiert. Die
    /// letzte Zeile wird komplett gelöscht und die Spaltenposition
    /// auf 0 zurück gesetzt.
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

    /// Löscht eine Zeile, indem alle Zeichen als ' '
    /// mit der intern gespeicherten Farkbodierung
    /// überschrieben werden.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    /// Schreibt eine übergebene Zeichenfolge in den Puffer.
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
}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    /// Statische Instanz des VGA-Buffer-Objekts.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// Druckt die übergebenen Zeichen auf den Bildschirm.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

/// Druckt die übergebenen Zeichen auf den Bildschirm mit einem \n.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    let s = "Some test string that fits on a single line";

    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{s}").expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
