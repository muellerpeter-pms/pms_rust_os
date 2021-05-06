
const TEST_EXIT_DEVICE_PORT : u16 = 0xf4;

use crate::{print, println};

/// das Resultat der Tests
#[derive(Debug)]
#[repr(u32)]
pub enum TestResult {
    Success = 0x01,
    Failed = 0x10
}

/// Beendet Qemu und sendet das Resultat des Tests als Rückkehrcode
pub fn test_exit_qemu( result : TestResult) -> ! {
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

/// Durchläuft die übergebenen Tests, danach beendet es qemu durch 'test_exit_qemu' mit TestResult::Success
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Starte {} tests", tests.len());
    for test in tests {
        test.run();
    }
    loop {}
    test_exit_qemu( TestResult::Success );
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T where T: Fn() {
    fn run (&self) 
    {
        print!( "{}...\t", core::any::type_name::<T>() );
        self();
        println!( "[ok]");
    }
}
