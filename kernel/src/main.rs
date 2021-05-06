#![no_std]
#![no_main]

#![feature(asm)]

mod vga_text;

use core::panic::PanicInfo;
use bootloader::BootInfo;


static HELLO : &[u8] = b"Hallo Welt\n";

#[no_mangle]
pub extern "C" fn _start( _boot_info : &BootInfo) -> ! { 
    
    vga_text::test_print();

    loop {
        unsafe {asm!("hlt"); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
