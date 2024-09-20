use crate::kernel_console::KernelConsole;
use crate::memory_management::free_memory_map::{AllocationType, FreeMemoryMap};
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
pub struct PaginationInfo {
    pub virtual_address: usize,
    pub physical_address: usize,
    pub size: usize,
    pub is_enabled: bool,
}
impl PaginationL4 {
    pub fn get_kernel_pagination() -> *mut PaginationL4 {
        return 0x1000 as *mut PaginationL4;
    }
    pub fn get(&self, virtual_ptr: usize) -> PaginationInfo {
        let entry_index = (virtual_ptr >> 39) & 0x1ff;
        if self.entries[entry_index] & 1 == 0 {
            return PaginationInfo {
                virtual_address: virtual_ptr & 0xffffff80_00000000,
                size: 0x80_00000000,
                is_enabled: false,
                physical_address: 0,
            };
        } else {
            unsafe {
                let l3 = (self.entries[entry_index] & 0xffffffff_fffff000) as *mut PaginationL3;
                return (*l3).get(virtual_ptr);
            }
        }
    }
    pub fn set(&mut self, info: PaginationInfo) {
        let entry_index = (info.virtual_address >> 39) & 0x1ff;
        if self.entries[entry_index] & 1 == 0 {
            let new_page = FreeMemoryMap::allocate_one_page(AllocationType::Allocated as u8);
            self.entries[entry_index] = new_page as u64 | 0x03;
        }
        unsafe {
            let l3 = (self.entries[entry_index] & 0xffffffff_fffff000) as *mut PaginationL3;
            (*l3).set(info);
        }
    }
}
impl PaginationL3 {
    pub fn get(&self, virtual_ptr: usize) -> PaginationInfo {
        let entry_index = (virtual_ptr >> 30) & 0x1ff;
        if self.entries[entry_index] & 1 == 0 {
            return PaginationInfo {
                virtual_address: virtual_ptr & 0xffffffff_c0000000,
                size: 0x40000000,
                is_enabled: false,
                physical_address: 0,
            };
        } else {
            unsafe {
                let l2 = (self.entries[entry_index] & 0xffffffff_fffff000) as *mut PaginationL2;
                return (*l2).get(virtual_ptr);
            }
        }
    }
    pub fn set(&mut self, info: PaginationInfo) {
        let entry_index = (info.virtual_address >> 30) & 0x1ff;
        if self.entries[entry_index] & 1 == 0 {
            let new_page = FreeMemoryMap::allocate_one_page(AllocationType::Allocated as u8);
            self.entries[entry_index] = new_page as u64 | 0x03;
        }
        unsafe {
            let l2 = (self.entries[entry_index] & 0xffffffff_fffff000) as *mut PaginationL2;
            (*l2).set(info);
        }
    }
}

impl PaginationL2 {
    pub fn get(&self, virtual_ptr: usize) -> PaginationInfo {
        let entry_index = (virtual_ptr >> 21) & 0x1ff;
        if self.entries[entry_index] & 1 == 0 {
            return PaginationInfo {
                virtual_address: virtual_ptr & 0xffffffff_ffe00000,
                size: 0x200000,
                is_enabled: false,
                physical_address: 0,
            };
        } else if self.entries[entry_index] & 0x80 == 0x80 {
            return PaginationInfo {
                virtual_address: virtual_ptr & 0xffffffff_ffe00000,
                size: 0x200000,
                is_enabled: true,
                physical_address: (self.entries[entry_index] & 0xffffffff_ffe00000) as usize,
            };
        } else {
            unsafe {
                let l1 = (self.entries[entry_index] & 0xffffffff_fffff000) as *mut PaginationL1;
                return (*l1).get(virtual_ptr);
            }
        }
    }

    pub fn set(&mut self, info: PaginationInfo) {
        let entry_index = (info.virtual_address >> 21) & 0x1ff;
        if info.size == 0x200000 {
            self.entries[entry_index] = info.physical_address as u64 & 0xffff_ffff_ffe00000 | 0x83
        } else if info.size == 0x1000
        {
            if self.entries[entry_index] & 1 == 0 {
                let new_page = FreeMemoryMap::allocate_one_page(AllocationType::Allocated as u8);
                self.entries[entry_index] = new_page as u64 | 0x03;
            }
            unsafe {
                let l1 = (self.entries[entry_index] & 0xffffffff_fffff000) as *mut PaginationL1;
                (*l1).set(info);
            }
        } else {
            panic!("unsupported page size")
        }
    }
}


impl PaginationL1 {
    pub fn get(&self, virtual_ptr: usize) -> PaginationInfo {
        let entry_index = (virtual_ptr >> 21) & 0x1ff;
        KernelConsole::printu64hex(self as *const PaginationL1 as u64);
        KernelConsole::print("get_mapping L1");
        KernelConsole::printu64hex(self.entries[entry_index]);
        KernelConsole::print("\n");
        KernelConsole::printu64hex(self.entries[entry_index] & 1);
        if self.entries[entry_index] & 1 == 0 {
            return PaginationInfo {
                virtual_address: virtual_ptr & 0xffffffff_fffff000,
                size: 0x1000,
                is_enabled: false,
                physical_address: 0,
            };
        } else {
            return PaginationInfo {
                virtual_address: virtual_ptr & 0xffffffff_fffff000,
                size: 0x1000,
                is_enabled: true,
                physical_address: (self.entries[entry_index] & 0xffffffff_fffff000) as usize,
            };
        }
    }
    pub fn set(&mut self, info: PaginationInfo) {
        let entry_index = (info.virtual_address >> 12) & 0x1ff;
        KernelConsole::print("set_mapping L1");
        KernelConsole::printu64hex(self.entries[entry_index]);
        KernelConsole::print("\n");
        KernelConsole::printu64hex((info.physical_address as u64 & 0xffff_ffff_ffff_f000) | 0x03);

        KernelConsole::print("\n");
        self.entries[entry_index] = (info.physical_address as u64 & 0xffff_ffff_ffff_f000) | 0x03;

        KernelConsole::printu64hex(self.entries[entry_index]);
    }
}