

/// Ein Wrapper um die ```in```-und ```out```-Instruktionen in Assembler
pub struct Port {
    address: u16,
}

impl Port {
    /// Erzeugt einen neuen Port für die angegebene Adresse
    pub fn new( address : u16) -> Port
    {
        Port {
            address
        }
    }

    /// Schreibt data auf dem Port
    pub fn write_u8 (  &self, data : u8 ) {
        unsafe {
        asm!("out dx, al", 
             in("dx") self.address,
             in("al") data);
        }
    } 

    /// Schreibt data auf dem Port
    pub fn write_u16 (  &self, data : u16 ) {
        unsafe {
        asm!("out dx, ax", 
             in("dx") self.address,
             in("ax") data);
        }
    } 

    /// Schreibt data auf dem Port
    pub fn write ( &self, data : u32 ) {
        unsafe {
        asm!("out dx, eax", 
                in("dx") self.address,
                in("eax") data);
        }
    } 

    /// Liest von dem Port
    pub fn read_u8 ( &self) -> u8 {
        let mut data:u8;
        unsafe {
        asm!("in al, dx", 
                in("dx") self.address,
                out("al") data);
        }
        data 
    } 

    /// Liest von dem Port
    pub fn read_u16 ( &self) -> u16 {
        let mut data:u16;
        unsafe {
        asm!("in ax, dx", 
             in("dx") self.address,
             out("ax") data);
        }
        data 
    } 

    /// Liest von dem Port
    pub fn read ( &self ) -> u32 {
        let mut data:u32;
        unsafe {
        asm!("in eax, dx", 
             in("dx") self.address,
             out("eax") data);
        }
        data 
    } 
}