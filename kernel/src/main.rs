#![no_std]
#![no_main]

#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]


mod vga_text;
mod x86_64;
#[cfg(test)]
mod testing;

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

    panic!("test!");

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    #[cfg(test)]
    println_serial!("{}", info);
    #[cfg(not(test))]
    println!("{}", info);
    
    #[cfg(test)]
    testing::test_exit_qemu( testing::TestResult::Failed );

    loop {}
}



#[cfg(test)]
mod tests {    
    #[test_case]
    fn simpler_test() { // ein belangloser Test, um unsere Testumgebung selbst zu testen
        assert_eq!(1, 1);
    }
}