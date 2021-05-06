#![no_std]
#![no_main]
#![feature(asm)]

mod vga_text;
mod x86_64;

use bootloader::BootInfo;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start(_boot_info: &BootInfo) -> ! {

    // Bildschirm leeren und Hintergrundfarbe setzen
    vga_text::WRITER.lock().clear();

    // Ein kurzes Hallo
    println!("Rust OS");
    println!("Version {}", env!("CARGO_PKG_VERSION"));

    panic!("test!");

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
