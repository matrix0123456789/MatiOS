use core::arch::asm;
use crate::kernel_console::KernelConsole;
use crate::process_management::process_table::ProcessTable;

pub struct InterruptHandlers{}
impl InterruptHandlers {
    #[naked]
    pub fn default_handler(){
        unsafe{
            asm!(
            "push rax",
            "push rbx",
            "push rcx",
            "push rdx",
            "push rsi",
            "push rdi",
            "push rbp",
            "push r8",
            "push r9",
            "push r10",
            "push r11",
            "push r12",
            "push r13",
            "push r14",
            "push r15",
            "mov rdi, rsp",
            "call default_handler_main",
            options(noreturn),
            )
        }
    }
    #[no_mangle]
     fn default_handler_main(stack_pointer_raw : u64){
         KernelConsole::print("Interrupt handled\n");
         ProcessTable::pause_current_thread(stack_pointer_raw);

        ProcessTable::resume_thread();
     }
}