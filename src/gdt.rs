//! Funktionen und Strukturen für die Global Descriptor Table.
//!
//! Die GDT ist historisch entstanden und wird in x86_64 nur noch in Teilen verwendet.
//! Besonders die TSS wird für einen speziellen Stack während Interrupts verwendet.

use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

/// Der Stack Index für Double Fault Exceptions.
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            /// Die Größe des Stacks
            const STACK_SIZE: usize = 4096 * 5;
            /// Der Stack für den Double Fault handler
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            #[allow(clippy::let_and_return)] // better readable as stack_end
            stack_end
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

/// Selektroen für Code-Segment und Task State Segment
struct Selectors {
    /// Code Segment Selector
    code_selector: SegmentSelector,
    /// Task State Segment Selector
    tss_selector: SegmentSelector,
}

/// Initialisiert die GDT.
///
/// Es wird die GDT als solche geladen.
/// CS: kernel code segment
/// Das task State Segment wird aktualisert.
pub fn init() {
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}
