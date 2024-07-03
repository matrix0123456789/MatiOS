#![no_std]
#![no_main]

use crate::kernel_console::{print};

mod kernel_console;
mod cpu_ports;

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
for i in 0..1000000 {

    print("Hello world in Rust\n");
    for i in 0..1000000 {

    }
}
}