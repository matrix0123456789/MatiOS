use crate::memory_management::bios_memory_map::BiosMemoryMapEntry;

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
/*
0 - free
1 - reserved
 */
pub fn init_memory_map() {
    let number_of_entries = last_useful_address() / 4096 + 1;
    unsafe{(*(0x100100 as *mut u64)) = number_of_entries;}
    let entries = 0x100108 as *mut u8;
    for i in 0..number_of_entries {
        if i * 4096 < 0x100000 { unsafe {
            *entries.offset(i as isize) = 1;
        }
        } else if !is_page_empty_by_bios_table(i) { unsafe {
            *entries.offset(i as isize) = 1;
        }
        } else if i * 4096 < 0x100108 + number_of_entries * 8 {
            unsafe {
                *entries.offset(i as isize) = 2;
            }
        } else {
            unsafe {
                *entries.offset(i as isize) = 0;
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
    let entries = 0x100108 as *mut u8;
    unsafe {
        let number_of_entries = *(0x100100 as *mut u64);
        let mut ret = 0;
        for i in 0..number_of_entries {

                if *entries.offset(i as isize) == 0 {
                    ret += 1;
                }

        }
        return ret;
    }
}