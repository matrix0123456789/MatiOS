use crate::kernel_console::{print, printu64hex};

pub struct BiosMemoryMapEntry {
    pub base_address: u64,
    pub length: u64,
    pub memory_type: u32,
    pub extended_attributes: u32
}

pub fn print_bios_map(){

    let bios_memory_map:*mut BiosMemoryMapEntry = 0x6000 as *mut BiosMemoryMapEntry;
    for i in 0..100 {
        let entry = unsafe { &*(bios_memory_map.offset( i)) };
        if entry.memory_type == 0 {
            break;
        }
        print("Base address: ");
        printu64hex(entry.base_address);
        print(" Length: ");
        printu64hex(entry.length);
        print(" Type: ");
        printu64hex(entry.memory_type as u64);
        print(" Extended attributes: ");
        printu64hex(entry.extended_attributes as u64);
        print("\n");
    }
}
