use core::alloc::{GlobalAlloc, Layout};
use crate::memory_management::free_memory_map::{AllocationType, FreeMemoryMap};

struct KernelHeap{

}
unsafe impl GlobalAlloc for KernelHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        return FreeMemoryMap::allocate_one_page(AllocationType::Allocated as u8)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        FreeMemoryMap::free_page(ptr)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let page = self.alloc(layout);
        for i in 0..layout.size() {
            *(page.offset(i as isize)) = 0;
        }
        return page;
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        return ptr;
    }
}
#[global_allocator]
static ALLOCATOR:KernelHeap=KernelHeap{};