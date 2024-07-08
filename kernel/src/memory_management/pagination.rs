use crate::memory_management::free_memory_map::FreeMemoryMap;
use crate::panic;

pub static KERNEL_PAGINATION: *mut PaginationL4 = 0x1000 as *mut PaginationL4;
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
        return KERNEL_PAGINATION;
    }
    pub fn set_mapping(self, virtual_address: u64, physical_address: u64) {
        let entry_index = (virtual_address >> 39) & 0x1ff;
        if self.entries[entry_index] & 1 == 0 {
            let new_page=FreeMemoryMap::get_free_page();
            self.entries[entry_index] = new_page | 0b11;
        }
        let l3 = (self.entries[entry_index] & 0x000fffff_fffff000) as *mut PaginationL3;
        l3.set_mapping(virtual_address && 0x0000fffffffff000, physical_address);
    }
}
impl PaginationL3{
    pub unsafe fn set_mapping(self, virtual_address: u64, physical_address: u64) {
        let entry_index = (virtual_address >> 30) & 0x1ff;
        if self.entries[entry_index] & 1 == 0 {
            let new_page=FreeMemoryMap::get_free_page();
            self.entries[entry_index] = new_page | 0b11;
        }
        let l2 = (self.entries[entry_index] & 0x000fffff_fffff000) as *mut PaginationL2;
        l2.set_mapping(virtual_address && 0x0000007ffffff000, physical_address);
    }
}

impl PaginationL2{
    pub unsafe fn set_mapping(self, virtual_address: u64, physical_address: u64) {
        let entry_index = (virtual_address >> 21) & 0x1ff;
        if self.entries[entry_index] & 1 == 0 {
            let new_page=FreeMemoryMap::get_free_page();
            self.entries[entry_index] = new_page | 0b11;
        }
        if self.entries[entry_index] & 0x80 != 0 {
            panic!("Not implemented");
        }
        let l1 = (self.entries[entry_index] & 0x000fffff_fffff000) as *mut PaginationL1;
        (*l1).set_mapping(virtual_address && 0x000000003ffff000, physical_address);
    }
}

impl PaginationL1{
    pub fn set_mapping(mut self, virtual_address: u64, physical_address: u64) {
        let entry_index = (virtual_address >> 12) & 0x1ff;
        (self).entries[entry_index] = physical_address | 0b11;
    }
}