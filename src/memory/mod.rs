use bootloader::bootinfo::MemoryMap;
use x86_64::{
    structures::paging::{page, PageTable, PageTableFlags, RecursivePageTable},
    VirtAddr,
};

mod physical;

pub use physical::PhysicalMemoryManager;

//use physical::PhysicalMemoryManager;

pub fn init(virt_memory_offset: u64, map: &MemoryMap) {
    let _pmm = PhysicalMemoryManager::init(map, virt_memory_offset);

    //  let mut rpt = RecursivePageTable::new(page_table).expect("Pagetable is not valid!");
    //  list_virtual_memory(page_table);
}
