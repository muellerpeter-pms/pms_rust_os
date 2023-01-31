use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    structures::paging::{FrameAllocator, Page, PageTable, PhysFrame, Size2MiB, Size4KiB},
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

    #[cfg(feature = "verbose")]
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

    fn register_pages_for_allocation_map(memory_map: &MemoryMap, virt_addr_offset: u64) {
        // get highest frame to map
        let highest_frame = Self::get_higest_physical_frame(memory_map);

        // each page can handle 0x8000 bits and therefore
        // 0x8000000 byte of memory in 4kiB sized frames
        const addresses_per_frame: u64 = 0x1000 * 8 * 0x1000;
        let frames_needed = (highest_frame * 0x1000 / addresses_per_frame) + 1;

        #[cfg(feature = "verbose")]
        serial_println!(
            "need {} pages for bitmapping physical ram with highest frame {}",
            frames_needed,
            highest_frame
        );
    }

    pub fn init(memory_map: &MemoryMap, virt_addr_offset: u64) -> Self {
        let physical_memory_manager = Self {};

        #[cfg(feature = "verbose")]
        Self::list_memory_regions(memory_map);

        Self::register_pages_for_allocation_map(memory_map, virt_addr_offset);

        physical_memory_manager
    }
}

unsafe impl FrameAllocator<Size4KiB> for PhysicalMemoryManager {
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size4KiB>> {
        None
    }
}

unsafe impl FrameAllocator<Size2MiB> for PhysicalMemoryManager {
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size2MiB>> {
        None
    }
}

/*
4 kiB
8 kiB
16 kiB
32 kiB
64 kiB
128 kiB
256 kiB
512 kiB
1 MiB
2 MiB
4 MiB
8 MiB
16 MiB
32 MiB
64 MiB
128 MiB
256 MiB
512 MiB
1 GiB
 */
