#![no_std]
#![no_main]

#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

const TEST_EXIT_DEVICE_PORT : u16 = 0xf4;

/// das Resultat der Tests
#[derive(Debug)]
#[repr(u32)]
enum TestResult {
    Success = 0x01,
    Failed = 0x10

}

fn test_exit_qemu( result : TestResult) -> ! {
    println!();
    println! ("Verlasse Qemu mit dem Resultat: {:?}", result);
    unsafe{        
        asm!("out dx, eax",        
        in("dx") TEST_EXIT_DEVICE_PORT,
        in("eax") result as u32,
        );
    }    

    println! ("Noch da? da ging was schief!"); // keine Panic, sonst sind wir in der Endlosschleife
    loop{unsafe{ asm!("hlt"); } }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Starte {} tests", tests.len());
    for test in tests {
        test();
    }
    test_exit_qemu( TestResult::Success );
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
    
    #[cfg(test)]
    test_exit_qemu( TestResult::Failed );

    loop {}
}



#[cfg(test)]
mod tests {    
    #[test_case]
    fn simpler_test() { // ein belangloser Test, um unsere Testumgebung selbst zu testen
        assert_eq!(1, 1);
    }
}