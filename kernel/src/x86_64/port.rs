

/// Ein Wrapper um die ```in```-und ```out```-Instruktionen in Assembler
pub struct Port {
    address: u16,
}

impl Port {
    /// Erzeugt einen neuen Port für die angegebene Adresse
    pub fn new( address : u16)
    {
        Port {
            address
        }
    }

    /// Schreibt data auf dem Port
    pub fn write (  &self, data : u32 ) {
        unsafe {
        asm!("out dx, eax", 
             in("dx") self.address,
             in("eax") data);
        }
    } 

    /// Liest von dem Port
    pub fn read (  &self) -> u32 {
        let mut data:u32;
        unsafe {
        asm!("in dx, eax", 
             in("dx") self.address,
             out("eax") data);
        }
    } 
}