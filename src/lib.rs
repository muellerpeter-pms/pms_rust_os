//! Peter Mueller Software - Rust Operating System
//!
//! Dieses System ist eine Hobbyarbeit zur Programmierung einer kleinen Kernel in der Programmiersprache Rust.
//! Dabei hält es sich im Grundaufbau an den Blog von Phillipp Oppermann <https://os.phil-opp.com/>
//!
//! Für den Betrieb des Systems müssen einige Vorruassetzungen am Entwicklungs-PC geschaffen werden:
//! - Das Betriebssystem sollte Windows sein.
//! - Installation von Qemu
//! - HAXM - muss installiert werden, mit den entsprechnenden Bedingungen (HyperV deaktiviern usw.)
//!

#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![deny(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

pub mod gdt;
pub mod interrupt;
pub mod memory;
pub mod serial;
pub mod vga_buffer;

use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::entry_point;
use bootloader::BootInfo;

/// Initialisierung des Prozessors und der Umgebung für den Kernel
///
/// Die Funktion initialisiert:
/// - GDT
/// - IDT , starte PICS, aktiviere
/// - Speicherverwaltung
pub fn init(boot_info: &'static BootInfo) {
    gdt::init();
    interrupt::init_idt();
    unsafe { interrupt::PICS.lock().initialize() }
    x86_64::instructions::interrupts::enable();

    memory::init(boot_info.physical_memory_offset, &boot_info.memory_map);
}

/// Halt-Loop für das energiesparende Stoppen eines Kerns
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// Trait für eine vereinfachte, allgemeine Testumgebung.
pub trait Testable {
    /// Startet die zugehörige Funktion.
    ///
    /// Dabei können vor und nach Ausführung zusätzliche Schritte ausgeführt werden, wie die
    /// Ausgabe des Namens oder der Ergebnisse des Tests.
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

/// Funktion für das Ausführen aller übergebenen Tests
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

/// Panic-Handler für die Ausführung der Tests in der lib.
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);

    hlt_loop();
}

#[cfg(test)]
entry_point!(kernel_test_main);
#[cfg(test)]
/// Entry point for `cargo test`
fn kernel_test_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
    test_main();

    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

/// Mögliche Exit-Codes für den Exit aus Qemu per Port 0xf4.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    /// Erfolg
    Success = 0x10,
    /// Fehler
    Failed = 0x11,
}

/// Funktion, um Qemu mit [Exit-Code](QemuExitCode) zu verlassen.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
