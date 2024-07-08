#![feature(panic_info_message)]
#![no_std]
#![no_main]

use crate::kernel_console::KernelConsole;
use crate::memory_management::free_memory_map::FreeMemoryMap;
use crate::memory_management::pagination::PaginationL4;

mod kernel_console;
mod cpu_ports;
mod memory_management;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    KernelConsole::set_color(0x17);
    KernelConsole::clear_screen();
KernelConsole::print("Blue Screen of Death\n");
    KernelConsole::print(_info.message().as_str().unwrap());
    KernelConsole::print("\n");
    KernelConsole::print(_info.location().unwrap().file());
    KernelConsole::print(":");
    KernelConsole::printu64dec(_info.location().unwrap().line() as u64);
    loop {}
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
    loop {}
}

#[link_section = ".main"]
fn main() {
    FreeMemoryMap::init_memory_map();
    KernelConsole::print("Hello world in Rust\n");

    PaginationL4::get_kernel_pagination().set_mapping(0x12340000, 0x4321000);
    PaginationL4::get_kernel_pagination().set_mapping(0x4321000, 0x4321000);


    loop {
        KernelConsole::print(">");
        let mut line=['\0';16];
        let mut index = 0;
        loop {
            let char = KernelConsole::read_and_wait();
            if char == '\n' {
                KernelConsole::print("\n");

                if line[0] == 'm' && line[1] == 'e' && line[2]== 'm'{
                    KernelConsole::printu64dec(FreeMemoryMap::count_free_pages()*4);
                    KernelConsole::print ("KB free\n");
                }
                else if line[0] == 'b' && line[1]=='s' && line[2] == 'o' && line[3] == 'd'{
                    panic!("Manual BSOD")
                }else{
                    KernelConsole::print("Unknown command\n");
                }

                break;
            }
            if char != '\0' {
                KernelConsole::print_char(char);
                line[index]=char;
                index+=1;
            }
        }
    }
}
#[no_mangle]
pub fn  memcpy(dest: *mut u8, src: *const u8, n: usize) {
    unsafe {
        for i in 0..n {
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    }
}
#[no_mangle]
pub fn memset(dest: *mut u8, val: u8, n: usize) {
    unsafe {
        for i in 0..n {
            *dest.offset(i as isize) = val;
        }
    }
}