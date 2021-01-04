use bootloader::{BootInfo, bootinfo::MemoryRegionType};
use bootloader::bootinfo::MemoryMap;
use x86_64::{PhysAddr, VirtAddr, structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB, mapper::{MapToError, MapperFlush}, page_table::FrameError}};

static mut MAPPER: Option<OffsetPageTable<'static>> = None;
static mut ALLOCATOR: Option<BootInfoFrameAllocator> = None;

static ERROR_MESSAGE: &'static str = "pros::memory::init must be called before map_to is called";

pub fn init(boot_info: &'static BootInfo) {
    unsafe { init_unsafe(boot_info); }
}

unsafe fn init_unsafe(boot_info: &'static bootloader::BootInfo) {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = active_level4_table(phys_mem_offset);
    let mapper = OffsetPageTable::new(l4_table, phys_mem_offset);
    MAPPER = Some(mapper);

    ALLOCATOR = Some(BootInfoFrameAllocator {
        memory_map: &boot_info.memory_map,
        next: 0,
    })
}

pub fn map_to(page: Page, frame: PhysFrame, flags: PageTableFlags) 
    -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>>
{
    let mapper = unsafe { MAPPER.as_mut().expect(ERROR_MESSAGE) };
    let allocator = unsafe { ALLOCATOR.as_mut().expect(ERROR_MESSAGE) };
    unsafe { map_to_unsafe(mapper, allocator, page, frame, flags) }
}

unsafe fn map_to_unsafe(
    mapper: &mut impl Mapper<Size4KiB>, frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    page: Page, frame: PhysFrame, flags: PageTableFlags) 
    -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>>
{
    mapper.map_to(page, frame, flags, frame_allocator)
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