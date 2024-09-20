#![feature(panic_info_message)]
#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::String;
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


    loop {
        KernelConsole::print(">");
        let mut line=['\0';16];
        let mut index = 0;
        loop {
            let char = KernelConsole::read_and_wait();
            if char == '\n' {
                KernelConsole::print("\n");

                let line_string:String= line[0..index].iter().collect();

                if line_string=="mem"{
                    KernelConsole::printu64dec(FreeMemoryMap::count_free_pages()*4);
                    KernelConsole::print ("KB free\n");
                }
                else if line_string == "bsod"{
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
#[no_mangle]
pub fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    unsafe {
        for i in 0..n {
            if *s1.offset(i as isize) != *s2.offset(i as isize) {
                return 1;
            }
        }
    }
    return 0;
}