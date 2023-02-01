//! Handhabung des physischen und virtuellen Speichers.
//!
//! Das Modul enthält die Verwaltung des physischen, als auch des virtuellen Speichers.
//! Zusätzlich ist ein Allocator für den Kernel-eigenen Heap implementiert. Todo!()

use bootloader::bootinfo::MemoryMap;

mod physical;

pub use physical::PhysicalMemoryManager;

//use physical::PhysicalMemoryManager;

/// Initialisiert die Speicherverwaltung
pub fn init(virt_memory_offset: u64, map: &MemoryMap) {
    let _pmm = PhysicalMemoryManager::init(map, virt_memory_offset);

    //  let mut rpt = RecursivePageTable::new(page_table).expect("Pagetable is not valid!");
    //  list_virtual_memory(page_table);
}
