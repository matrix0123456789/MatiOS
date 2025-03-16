use crate::kernel_console::KernelConsole;

pub struct InterruptHandlers{}
impl InterruptHandlers {
    pub fn default_handler(){
        KernelConsole::print("Interrupt handled\n");
        loop {

        }
    }
}