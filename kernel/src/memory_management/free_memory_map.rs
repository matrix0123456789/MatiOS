use crate::memory_management::bios_memory_map::BiosMemoryMapEntry;

pub struct FreeMemoryMap {}
static mut NUMBER_OF_ENTRIES: u64 = 0;
static mut ENTRIES: *mut u8 = 0 as *mut u8;
#[repr(u8)]
pub enum AllocationType {
    Free = 0,
    Reserved = 1,
    Allocated = 2,
}
impl FreeMemoryMap {
    fn last_useful_address() -> u64 {
        let bios_memory_map: *mut BiosMemoryMapEntry = 0x6000 as *mut BiosMemoryMapEntry;
        let mut ret: u64 = 0;
        for i in 0..100 {
            let entry = unsafe { &*(bios_memory_map.offset(i)) };
            if entry.memory_type == 0 {
                break;
            }
            if entry.memory_type == 1 {
                let last_address = entry.base_address + entry.length;
                if last_address > ret {
                    ret = last_address;
                }
            }
        }
        return ret;
    }
    fn start_of_biggest_free_memory_block() -> u64 {
        let bios_memory_map: *mut BiosMemoryMapEntry = 0x6000 as *mut BiosMemoryMapEntry;
        let mut adress_of_biggest_block: u64 = 0;
        let mut size_of_biggest_block: u64 = 0;
        for i in 0..100 {
            let entry = unsafe { &*(bios_memory_map.offset(i)) };
            if entry.memory_type == 0 {
                break;
            }

            if entry.memory_type == 1 {
                if entry.length > size_of_biggest_block {
                    size_of_biggest_block = entry.length;
                    adress_of_biggest_block = entry.base_address;
                }
            }
        }
        return adress_of_biggest_block;
    }


    pub fn init_memory_map() {
        unsafe {
            NUMBER_OF_ENTRIES = FreeMemoryMap::last_useful_address() / 4096 + 1;

            ENTRIES = FreeMemoryMap::start_of_biggest_free_memory_block() as *mut u8;

            for i in 0..NUMBER_OF_ENTRIES {
                if !FreeMemoryMap::is_page_empty_by_bios_table(i) {

                    *ENTRIES.offset(i as isize) = AllocationType::Reserved as u8;

                } else if i * 4096 < 0x100000 {
                        *ENTRIES.offset(i as isize) = AllocationType::Allocated as u8;

                } else if i * 4096 < ENTRIES.offset(NUMBER_OF_ENTRIES as isize) as u64 {

                        *ENTRIES.offset(i as isize) = AllocationType::Allocated as u8;

                } else {

                        *ENTRIES.offset(i as isize) = AllocationType::Free as u8;

                }
            }
        }
    }
    fn is_page_empty_by_bios_table(page: u64) -> bool {
        let bios_memory_map: *mut BiosMemoryMapEntry = 0x6000 as *mut BiosMemoryMapEntry;
        for i in 0..100 {
            let entry = unsafe { &*(bios_memory_map.offset(i)) };
            if entry.memory_type == 0 {
                break;
            }
            if entry.memory_type == 1 {
                let last_address = entry.base_address + entry.length;
                if page * 4096 >= entry.base_address && (page + 1) * 4096 < last_address {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn count_free_pages() -> u64 {
        unsafe {
            let mut ret = 0;
            for i in 0..NUMBER_OF_ENTRIES {
                if *ENTRIES.offset(i as isize) == AllocationType::Free as u8{
                    ret += 1;
                }
            }
            return ret;
        }
    }

    pub fn allocate_one_page(allocation_type: u8) -> *mut u8 {
        unsafe {
            for i in 0..NUMBER_OF_ENTRIES {
                if *ENTRIES.offset(i as isize) == AllocationType::Free as u8 {
                    *ENTRIES.offset(i as isize) = allocation_type;
                    return (i * 4096) as *mut u8;
                }
            }
        }
        panic!("No free memory");
    }
    pub fn free_page(page: *mut u8) {
        unsafe {
            let page_number = page as u64 / 4096;
            if page_number >= NUMBER_OF_ENTRIES {
                panic!("Invalid page number");
            }
            if *ENTRIES.offset(page_number as isize) == AllocationType::Free as u8 {
                panic!("Page is already free");
            }
            *ENTRIES.offset(page_number as isize) = AllocationType::Free as u8;
        }
    }
}