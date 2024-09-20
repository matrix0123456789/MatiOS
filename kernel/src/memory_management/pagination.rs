use crate::memory_management::free_memory_map::FreeMemoryMap;
use crate::panic;

pub struct PaginationL1 {
    pub entries: [u64; 512],
}
pub struct PaginationL2 {
    pub entries: [u64; 512],
}
pub struct PaginationL3 {
    pub entries: [u64; 512],
}
pub struct PaginationL4 {
    pub entries: [u64; 512],
}

impl PaginationL4 {
    pub fn get_kernel_pagination() -> *mut PaginationL4 {
        return 0x1000 as *mut PaginationL4;
    }

}
impl PaginationL3{

}

impl PaginationL2{

}

impl PaginationL1{

}