use x86_64::{
    structures::paging::{PageTable, RecursivePageTable},
    VirtAddr,
};

#[cfg(feature = "verbose")]
use crate::println;

pub fn init(page_table_address: u64) -> RecursivePageTable<'static> {
    let page_table: &mut PageTable = unsafe { get_reference_to_page_table(page_table_address) };

    RecursivePageTable::new(page_table).expect("Pagetable is not valid!")
}

unsafe fn get_reference_to_page_table(address: u64) -> &'static mut PageTable {
    let virt = VirtAddr::new(address);

    #[cfg(feature = "verbose")]
    println!("found recursive memory table at : {:?}", virt);

    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    &mut *page_table_ptr
}
