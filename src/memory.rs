use bootloader::{BootInfo, bootinfo::MemoryRegionType};
use bootloader::bootinfo::MemoryMap;
use x86_64::{PhysAddr, VirtAddr, structures::paging::{Page, Mapper, FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB, page_table::FrameError}};

pub unsafe fn init(boot_info: &'static BootInfo) -> OffsetPageTable<'static> {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = active_level4_table(phys_mem_offset);
    OffsetPageTable::new(l4_table, phys_mem_offset)
}

pub unsafe fn active_level4_table(
    phys_mem_offset: VirtAddr) -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level4_table_frame, _) = Cr3::read(); 
    let level4_table_start_phys = level4_table_frame.start_address();
    let level4_table_start_virt =
        phys_mem_offset + level4_table_start_phys.as_u64();
    let page_table_ptr: *mut PageTable = level4_table_start_virt.as_mut_ptr();

    &mut *page_table_ptr
}

pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>)
{
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)
    };

    map_to_result.expect("map_to failed").flush();
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}