
//! Hier wird die serielle Schnittstelle auf Basis der Befehle für [UART16550](https://wiki.osdev.org/Serial_ports) 
//! eingerichtet.
//! Außerdem können werden AMkros für das Senden über die Schnittstelle bereit gestellt.

use super::Port;

pub struct UART {
    base_address: u16,
}



impl UART {
    pub fn new( base_address : u16 ) -> UART {
       UART { 
           base_address 
        }
    }

    pub fn init( &self ) {  
        let base = self.base_address;
        
        let p0 = Port::new( base + 0x00 );
        let p1 = Port::new( base + 0x01 );
        let p2 = Port::new( base + 0x02 );
        let p3 = Port::new( base + 0x03 );
        let p4 = Port::new( base + 0x04 );

        p1.write_u8( 0x00); // Alle Interrupts aus
        p3.write_u8( 0x80); // DLAB -> setze Baudrate
        p0.write_u8( 0x03); // Teiler 3 low -> 38400 baud
        p1.write_u8( 0x00); //          high
        p3.write_u8( 0x03); // 8 bits, no parity, one stop bit
        p2.write_u8( 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        p4.write_u8( 0x0B); // IRQs enabled, RTS/DSR set
        p4.write_u8( 0x1E); // Set in loopback mode, test the serial chip

        p0.write_u8( 0xAE); // Test serial chip (send byte 0xAE and check if serial returns same byte)

        if p0.read_u8() != 0xAE {
            panic!("serieller Port konnte nicht initialisiert werden!");
        }

        // normaler Modus
        p4.write_u8( 0x0F);        
    }


    pub fn received( &self ) -> bool {

        let p5 = Port::new( self.base_address + 0x05);
        let p5_data = p5.read_u8();
        
        if (p5_data & 1) > 0 {
            return true;
        }
        return false;
     }
      
     pub fn read( &self ) -> u8 {
        while !self.received() {}
      
        let p0 = Port::new( self.base_address + 0x00);
        return p0.read_u8();
     }

     pub fn is_transmit_empty( &self) -> bool {
        let p5 = Port::new( self.base_address + 0x05);
        let p5_data = p5.read_u8();
        
        if (p5_data & 0x20) > 0 {
            return true;
        }
        return false;
     }
      
     pub fn write( &self, byte : u8) {
        while !self.is_transmit_empty() {}
      
        let p0 = Port::new( self.base_address + 0x00);
        return p0.write_u8( byte );
     }

     pub fn write_string( &self, text : &str) {
        for byte in text.bytes() {
            match byte {
                // druckbare Zeichen
                0x20..=0x7e | b'\n' | b'\t' => self.write(byte),
                // außerhalb der druckbaren Zeichen
                _ => self.write(0xfe),
            }
        }
    }

}

use core::fmt;


impl fmt::Write for UART {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.write_string(text);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;


lazy_static! {
    pub static ref SERIAL : Mutex<UART> = {
        let serial = UART::new( 0x3f8 );
        serial.init();
        Mutex::new(serial)
    };
}

#[macro_export]
macro_rules! print_serial {
    ($($arg:tt)*) => ($crate::x86_64::uart_16550::_print_serial(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println_serial {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print_serial!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print_serial(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL.lock().write_fmt(args).unwrap();
}


