#[repr(C)]
pub struct InterruptDescriptorTableEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    attributes: u8,
    offset_middle: u16,
    offset_high: u32,
    zero: u32,
}
impl InterruptDescriptorTableEntry {
    pub fn new(fun_ptr: fn())->InterruptDescriptorTableEntry {
        let fun_ptr_int = fun_ptr as u64;
        Self {
            offset_low: fun_ptr_int as u16,
            selector: 0x20, // Code segment selector
            ist: 0,
            attributes: 0x8E, // Present, DPL=0, Type=14 (32-bit interrupt gate)
            offset_middle: (fun_ptr_int >> 16) as u16,
            offset_high: (fun_ptr_int >> 32) as u32,
            zero: 0,
        }
    }
}

impl Clone for InterruptDescriptorTableEntry {
    fn clone(&self) -> Self {
        return Self {
            offset_low: self.offset_low,
            selector: self.selector,
            ist: self.ist,
            attributes: self.attributes,
            offset_middle: self.offset_middle,
            offset_high: self.offset_high,
            zero: self.zero,
        }
    }
}

impl Copy for InterruptDescriptorTableEntry {}