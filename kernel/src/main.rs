#![no_std]
#![no_main]

use crate::kernel_console::KernelConsole;
use crate::memory_management::free_memory_map::FreeMemoryMap;

mod kernel_console;
mod cpu_ports;
mod memory_management;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {

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
    KernelConsole::printu64dec(FreeMemoryMap::count_free_pages()*4);
    KernelConsole::print ("KB free\n");

    KernelConsole::print("Allocated page:");
    KernelConsole::printu64hex(FreeMemoryMap::allocate_one_page(2) as u64);
    KernelConsole::print("\n");

    KernelConsole::print("Allocated page:");
    KernelConsole::printu64hex(FreeMemoryMap::allocate_one_page(2) as u64);
    KernelConsole::print("\n");


    KernelConsole::printu64dec(FreeMemoryMap::count_free_pages()*4);
    KernelConsole::print ("KB free\n");
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