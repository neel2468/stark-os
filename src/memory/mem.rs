use x86_64::{PhysAddr, VirtAddr, registers::control::Cr3, structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB}};

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    unsafe {
        let level_4_page_table = active_level_4_page_table(physical_memory_offset);
        OffsetPageTable::new(level_4_page_table, physical_memory_offset)
    }
}

unsafe fn active_level_4_page_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let (level_4_page_table_frame,_) = Cr3::read();
    let phys_addr = level_4_page_table_frame.start_address();
    let virt_addr = physical_memory_offset + phys_addr.as_u64();
    let page_table_ptr: *mut PageTable = virt_addr.as_mut_ptr();
    unsafe {
        &mut *page_table_ptr
    }
}

pub fn create_test_mapping(page: Page, mapper: &mut OffsetPageTable, frame_allocator: &mut impl FrameAllocator<Size4KiB>) {
    use x86_64::structures::paging::PageTableFlags as Flags;
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        //FIXME: this is not safe, it is only for testing
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}