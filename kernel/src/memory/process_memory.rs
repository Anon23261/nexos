use x86_64::{
    structures::paging::{
        PageTable, PageTableFlags, PhysFrame, Size4KiB,
        FrameAllocator, Mapper, Page, RecursivePageTable,
    },
    VirtAddr, PhysAddr,
};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

pub struct ProcessMemory {
    page_table: RecursivePageTable<'static>,
    next_free_frame: PhysFrame,
}

impl ProcessMemory {
    pub fn new(
        recursive_page_table: RecursivePageTable<'static>,
        memory_map: &'static MemoryMap,
    ) -> Self {
        ProcessMemory {
            page_table: recursive_page_table,
            next_free_frame: Self::init_frame_allocator(memory_map),
        }
    }

    pub fn map_program(&mut self, program_data: &[u8], load_addr: VirtAddr) -> Result<(), &'static str> {
        let mut current_page = Page::containing_address(load_addr);
        let pages_needed = (program_data.len() + 0xFFF) / 0x1000;

        // Map program pages
        for i in 0..pages_needed {
            let frame = self.allocate_frame()?;
            let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
            
            unsafe {
                self.map_page_to_frame(current_page, frame, flags)?;
                
                // Copy program data to frame
                let start = i * 0x1000;
                let end = core::cmp::min((i + 1) * 0x1000, program_data.len());
                let dest = frame.start_address().as_u64() as *mut u8;
                dest.copy_from(program_data[start..end].as_ptr(), end - start);
            }
            
            current_page += 1;
        }

        Ok(())
    }

    pub fn create_stack(&mut self, stack_bottom: VirtAddr, size_in_pages: usize) -> Result<VirtAddr, &'static str> {
        let stack_start = Page::containing_address(stack_bottom);
        
        // Map stack pages
        for i in 0..size_in_pages {
            let page = stack_start + i;
            let frame = self.allocate_frame()?;
            let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
            
            unsafe {
                self.map_page_to_frame(page, frame, flags)?;
            }
        }

        Ok(stack_bottom + (size_in_pages * 0x1000))
    }

    fn allocate_frame(&mut self) -> Result<PhysFrame, &'static str> {
        let frame = self.next_free_frame;
        self.next_free_frame += 1;
        Ok(frame)
    }

    unsafe fn map_page_to_frame(
        &mut self,
        page: Page<Size4KiB>,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
    ) -> Result<(), &'static str> {
        self.page_table
            .map_to(page, frame, flags, &mut FrameAllocator::new())
            .map_err(|_| "Failed to map page to frame")?
            .flush();
        Ok(())
    }

    fn init_frame_allocator(memory_map: &MemoryMap) -> PhysFrame {
        // Find the first usable memory region
        let region = memory_map
            .iter()
            .find(|r| r.region_type == MemoryRegionType::Usable)
            .expect("No usable memory regions found");

        PhysFrame::containing_address(PhysAddr::new(region.range.start_addr()))
    }
}

struct FrameAllocator;

impl FrameAllocator {
    fn new() -> Self {
        FrameAllocator
    }
}
