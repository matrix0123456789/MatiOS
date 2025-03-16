use crate::memory_management::pagination::{PaginationInfo, PaginationL4};

pub struct LocalApic {}
enum LocalApicRegister {
    ID = 0x20,
    Version = 0x30,
    EOI = 0xb0,
    SpuriousInterruptVector = 0xf0,
    Timer = 0x320,
    TimerInitialCount = 0x380,
    TimerCurrentCount = 0x390,
    TimerDivideConfig = 0x3e0,
}
const LOCAL_APIC_BASE: usize = 0xfee00000;
impl LocalApic {
    fn set_pagination() {
        let info = PaginationInfo {
            physical_address: LOCAL_APIC_BASE,
            virtual_address: LOCAL_APIC_BASE,
            size: 0x1000,
            is_enabled: true,
        };
        unsafe {
            (*PaginationL4::get_kernel_pagination()).set(info);
        }
    }
    pub fn init() {
        Self::set_pagination();
    }
    pub fn reset_timer() {
        Self::write(LocalApicRegister::EOI, 0);
        Self::write(LocalApicRegister::TimerInitialCount, 1000000000);
        Self::write(LocalApicRegister::Timer, 32);
        Self::write(LocalApicRegister::TimerDivideConfig, 0)
    }
    fn read(reg: LocalApicRegister) -> u32 {

        Self::set_pagination();
        unsafe {
            let ptr = (LOCAL_APIC_BASE as usize + reg as usize) as *const u32;
            return *ptr;
        }
    }
    fn write(reg: LocalApicRegister, value: u32) {

        Self::set_pagination();
        unsafe {
            let ptr = (LOCAL_APIC_BASE as usize + reg as usize) as *mut u32;
            *ptr = value;
        }
        Self::read(LocalApicRegister::ID);
    }
}
