//! Die Verwaltung des physischen Speichers.

use super::Locked;
use bootloader::bootinfo::{MemoryMap, MemoryRegion, MemoryRegionType};
use lazy_static::lazy_static;
use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame, Size2MiB, Size4KiB},
    PhysAddr,
};

#[cfg(feature = "verbose")]
use crate::serial_println;

/// The physical allocator, responsible for paged access to
/// physical memory.
/// The allocator uses a bitmap of pages and therefore itself reserves
/// some pages of memory during initialization.
pub struct PhysicalMemoryManager {
    /// Enthält die Memory Map aus dem Bootloader
    memory_map: Option<&'static MemoryMap>,
    /// Anzahl der einfach ausgegebenen Seiten
    simple_allocated_pages: usize,
}

lazy_static! {
    /// Statische Instanz der physischen Speicherverwaltung.
    pub static ref PMM: Locked<PhysicalMemoryManager> = Locked::new(PhysicalMemoryManager {
        memory_map: None,
        simple_allocated_pages: 0,
    });
}

impl Locked<PhysicalMemoryManager> {
    /// Funktion, welche alle von der Bootinfo übergebenen Speicherregionen auflistet.
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

    /// Initialisiert die physische Speicherverwaltung
    ///
    /// Es wird der Offset für den komlett gemappten physischen Adressraum benötigt,
    /// sowie auch der MemoryMap aus dem Bootloader. Diese werden intern gespeichert.
    pub fn init(&self, _virt_addr_offset: u64, memory_map: &'static MemoryMap) {
        let mut lock = self.lock();
        lock.memory_map = Some(memory_map);

        #[cfg(feature = "verbose")]
        Self::list_memory_regions(memory_map);
    }

    /// Erzeug einen Iterartor über die nutzbaren [MemoryRegion] aus der Bootloaderinfo.
    fn get_usable_regions_from_memory_map(&self) -> impl Iterator<Item = &MemoryRegion> {
        let pmm = self.lock();
        match pmm.memory_map {
            Some(mm) => {
                let regions = mm.iter();
                regions.filter(|r| r.region_type == MemoryRegionType::Usable)
            }
            None => panic!("No memory map is initialized!"),
        }
    }

    /// Erzeugt einen Iterarot über die nutzbaren Seiten<4KiB>
    fn get_page_iterator_from_memory_map(&self) -> impl Iterator<Item = PhysFrame> + '_ {
        let usable_regions = self.get_usable_regions_from_memory_map();
        // map each region to its address range
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }

    /// Gibt eine einzelne Seite<4KiB> zurück.
    fn pop_simple_allocated_page(&self) -> PhysFrame {
        let mut pages = self.get_page_iterator_from_memory_map();
        let mut pmm = self.lock();
        let page = pages
            .nth(pmm.simple_allocated_pages)
            .expect("Out of Memory!");
        pmm.simple_allocated_pages += 1;
        page
    }
}

unsafe impl FrameAllocator<Size4KiB> for Locked<PhysicalMemoryManager> {
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size4KiB>> {
        Some(self.pop_simple_allocated_page())
    }
}

unsafe impl FrameAllocator<Size2MiB> for PhysicalMemoryManager {
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame<Size2MiB>> {
        None
    }
}
