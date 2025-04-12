use crate::interrupts::interrupt_descriptoy_table::InterruptDescriptorTable;

#[repr(C, packed)]
pub struct InterruptDescriptorTablePointer {
    pub limit: u16,
    pub base: *const InterruptDescriptorTable,
}