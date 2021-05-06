#![no_std]
#![no_main]

#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Starte {} tests", tests.len());
    for test in tests {
        test();
    }
}

mod vga_text;

use bootloader::BootInfo;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start(_boot_info: &BootInfo) -> ! {

    // Bildschirm leeren und Hintergrundfarbe setzen
    vga_text::WRITER.lock().clear();

#[cfg(test)]
    test_main();

    // Ein kurzes Hallo
    println!("Rust OS");
    println!("Version {}", env!("CARGO_PKG_VERSION"));

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



#[cfg(test)]
mod tests {    
    #[test_case]
    fn simpler_test() { // ein belangloser Test, um unsere Testumgebung selbst zu testen
        assert_eq!(1, 1);
    }
}