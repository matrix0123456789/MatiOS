use core::arch::asm;
use crate::kernel_console::KernelConsole;

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
            "call default_handler_main",
            options(noreturn),
            )
        }
    }
    #[no_mangle]
     fn default_handler_main(){
         KernelConsole::print("Interrupt handled\n");
         loop {

         }
     }
}