use crate::interrupts::interrupt_descriptor_table_entry::InterruptDescriptorTableEntry;
use crate::interrupts::interrupt_descriptor_table_pointer::InterruptDescriptorTablePointer;
use crate::interrupts::interrupt_handlers::InterruptHandlers;
use core::arch::asm;

#[repr(C, align(16))]
pub struct InterruptDescriptorTable {
    entries: [InterruptDescriptorTableEntry; 256],
}
impl InterruptDescriptorTable {
    pub fn new() -> InterruptDescriptorTable {
        return InterruptDescriptorTable {
            entries: [InterruptDescriptorTableEntry::new(InterruptHandlers::default_handler); 256],
        };
    }
    pub fn init() {
        let table = InterruptDescriptorTable::new();
        let idt_pointer = InterruptDescriptorTablePointer {
            limit: (core::mem::size_of::<InterruptDescriptorTable>() - 1) as u16,
            base: &table,
        };
        unsafe {
            asm!(
            "lidt[{}]",
            "sti",
            "int 0",
            in(reg) &idt_pointer,
            )
        }
    }
}
