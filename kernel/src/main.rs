#![no_std]
#![no_main]

use crate::kernel_console::{print, printu64dec, printu64hex};
use crate::memory_management::bios_memory_map::print_bios_map;
use crate::memory_management::free_memory_map::{allocate_one_page, count_free_pages, init_memory_map};

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
init_memory_map();
    print("Hello world in Rust\n");
    printu64dec(count_free_pages()*4);
    print ("KB free\n");

    print("Allocated page:");
    printu64hex(allocate_one_page(2) as u64);
    print("\n");

    print("Allocated page:");
    printu64hex(allocate_one_page(2) as u64);
    print("\n");


    printu64dec(count_free_pages()*4);
    print ("KB free\n");
}