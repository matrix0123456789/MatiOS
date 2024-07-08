use crate::kernel_console::KernelConsole;

pub struct BiosMemoryMap{

}
pub struct BiosMemoryMapEntry {
    pub base_address: u64,
    pub length: u64,
    pub memory_type: u32,
    pub extended_attributes: u32
}
impl BiosMemoryMap {
    pub fn print_bios_map() {
        let bios_memory_map: *mut BiosMemoryMapEntry = 0x6000 as *mut BiosMemoryMapEntry;
        for i in 0..100 {
            let entry = unsafe { &*(bios_memory_map.offset(i)) };
            if entry.memory_type == 0 {
                break;
            }
            KernelConsole::print("Base address: ");
            KernelConsole::printu64hex(entry.base_address);
            KernelConsole::print(" Length: ");
            KernelConsole::printu64hex(entry.length);
            KernelConsole::print(" Type: ");
            KernelConsole::printu64hex(entry.memory_type as u64);
            KernelConsole::print(" Extended attributes: ");
            KernelConsole::printu64hex(entry.extended_attributes as u64);
            KernelConsole::print("\n");
        }
    }
}