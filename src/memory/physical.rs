use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
    PhysAddr,
};

#[cfg(feature = "verbose")]
use crate::serial_println;

/// The physical allocator, responsible for paged access to
/// physical memory.
/// The allocator uses a bitmap of pages and therefore itself reserves
/// some pages of memory during initialization.
pub struct PhysicalMemoryManager {}

impl PhysicalMemoryManager {
    // get iterator over frames
    fn get_frame_iterator(memory_map: &MemoryMap) -> impl Iterator<Item = u64> + '_ {
        // get regions as iteration
        let regions = memory_map.iter();
        // filter for usable regions
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        // get iterator for adress-ranges
        let frames =
            usable_regions.flat_map(|r| r.range.start_frame_number..r.range.end_frame_number);

        frames
    }

    fn get_higest_physical_frame(memory_map: &MemoryMap) -> u64 {
        // get regions as iteration
        let regions = memory_map.iter();
        // filter for usable regions
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);

        let last_region = usable_regions
            .last()
            .expect("There is no usable memory region!");

        let lastframe = last_region.range.end_frame_number - 1;

        lastframe
    }

    fn list_memory_regions(memory_map: &MemoryMap) {
        let regions = memory_map.iter();
        // filter for usable

        serial_println!("bootinfo contained this list of memory regions:");

        serial_println!("\t{:>12} \t{:>12}\t{:>8}   \tType", "from", "to", "size");

        regions.for_each(|r| {
            serial_println!(
                "\t {:#12x}\t{:#12x} :{:>8}kiB, \t {:?}",
                r.range.start_addr(),
                r.range.end_addr(),
                (r.range.end_frame_number - r.range.start_frame_number) * 4,
                r.region_type,
            )
        });
    }

    fn register_pages_for_allocation_map {
        // get highest frame to map
        let _highest_frame = Self::get_higest_physical_frame(memory_map);

    }

    pub fn init(memory_map: &MemoryMap) -> Self {
        let physical_memory_manager = Self {};

        #[cfg(feature = "verbose")]
        Self::list_memory_regions(memory_map);

        //let iterator = Self::get_frame_iterator(map);

        physical_memory_manager
    }
}

unsafe impl FrameAllocator<Size4KiB> for PhysicalMemoryManager {
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size4KiB>> {
        None
    }
}
