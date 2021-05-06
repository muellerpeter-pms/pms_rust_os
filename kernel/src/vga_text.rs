//! Enthält Funktionen für das Drucken von Zeichen auf dem Bildschirm
//! 
//! Die Ausführung orientiert sich an den Informationen von [wiki.osdev.org](https://wiki.osdev.org/Printing_to_Screen)


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
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Eine Struktur zum Darstellen von Zeichen auf dem Bildschirm
pub struct Writer {
    column_position: usize,
    line_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {

    fn write_char (&mut self, c : char) {
        const tab_width : usize = 4;
        match c {
            '\n' => self.new_line(),   // neue Zeile 
            '\t' => {                  // Tab
                let tab_pos = self.column_position % tab_width; // Position im tab berechnen
                self.column_position+= tab_width - tab_pos; // Vorrücken zum nächsten tab
            }, 
            c => {
                if self.column_position >= BUFFER_WIDTH { // Neue Zeile bei Bedarf
                    self.new_line();
                }
                
                self.buffer.chars [self.line_position][ self.column_position] =
                    ScreenChar {
                        ascii_character : c as u8,
                        color_code : self.color_code,
                    };
                
                self.column_position += 1;
            }
        }
    }

    fn new_line( &mut self) {
        self.line_position += 1;
        self.column_position = 0;
        
        if self.line_position >= BUFFER_HEIGHT {
            self.scroll_up();
        }
    }

    fn scroll_up( &mut self) {
        // Alle Zeilen nach oben rücken 
        for y in 1.. BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                self.buffer.chars [y-1][x] = self.buffer.chars[y][x];
            }
        }

        // letzte Zeile leeren
        let empty = ScreenChar {
            ascii_character : b' ',
            color_code: self.color_code,
        };
        for x in 0..BUFFER_WIDTH {
            self.buffer.chars [BUFFER_HEIGHT-1][x] = empty;
        }

        self.line_position = BUFFER_HEIGHT - 1;
    }

    pub fn clear( &mut self) {
        let empty = ScreenChar {
            ascii_character : b' ',
            color_code: self.color_code,
        };
        for y in 0.. BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                self.buffer.chars [y][x] = empty;
            }
        }
        self.line_position = 0;
        self.column_position = 0;
    }

    pub fn print_string( &mut self, text : &str) {
        for byte in text.bytes() {
            match byte {
                // druckbare Zeichen
                0x20..=0x7e | b'\n' | b'\t' => self.write_char(byte as char),
                // außerhalb der druckbaren Zeichen
                _ => self.write_char(  0xfe as char ),
            }

        }
    }
}

pub fn test_print () {
    let mut writer = Writer{
        column_position: 0,
        line_position: 0,
        color_code: ColorCode::new( Color::Black, Color::White),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },

    };


    writer.clear();
    writer.print_string ("Hallo von der kernel!\n");
    writer.print_string ("\tim ersten Tab\n");
    writer.print_string ("1\tim ersten Tab\n");
}