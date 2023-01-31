#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(pms_rust_os::test_runner)]

use core::panic::PanicInfo;
use pms_rust_os::{hlt_loop, println};

use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    pms_rust_os::init(boot_info);

    #[cfg(test)]
    test_main();

    hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pms_rust_os::test_panic_handler(info);
}

#[test_case]
#[allow(clippy::eq_op)]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
