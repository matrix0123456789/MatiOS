#![feature(panic_info_message)]
#![feature(naked_functions)]
#![no_std]
#![no_main]
extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use crate::drivers::bus::pci::PciDevice;
use crate::interrupts::interrupt_descriptoy_table::InterruptDescriptorTable;
use crate::kernel_console::KernelConsole;
use crate::memory_management::free_memory_map::FreeMemoryMap;
use crate::memory_management::pagination::{PaginationInfo, PaginationL4};
use crate::process_management::process_table::ProcessTable;

mod kernel_console;
mod cpu_ports;
mod memory_management;
mod drivers;
mod interrupts;
mod process_management;

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
ProcessTable::add_kernel_process();

    loop {
        KernelConsole::print(">");
        let mut line = ['\0'; 256];
        let mut index = 0;
        loop {
            let char = KernelConsole::read_and_wait();
            if char == '\n' {
                KernelConsole::print("\n");

                let line_string: String = line[0..index].iter().collect();

                if line_string == "mem" {
                    KernelConsole::printu64dec(FreeMemoryMap::count_free_pages() * 4);
                    KernelConsole::print("KB free\n");
                } else if line_string == "bsod" {
                    panic!("Manual BSOD")
                } else if line_string.starts_with("getpage ") {
                    let address = usize::from_str_radix(&line_string[8..], 16).unwrap();
                    unsafe {
                        let info = (*PaginationL4::get_kernel_pagination()).get(address);
                        KernelConsole::print("Virtual address: ");
                        KernelConsole::printu64hex(info.virtual_address as u64);
                        KernelConsole::print("\nSize: ");
                        KernelConsole::printu64hex(info.size as u64);
                        if info.is_enabled {
                            KernelConsole::print("\nPhysical address: ");
                            KernelConsole::printu64hex(info.physical_address as u64);
                        } else {
                            KernelConsole::print("\nPage not mapped");
                        }
                    }
                } else if line_string.starts_with("setpage ") {
                    let split_line = line_string.split(" ");
                    let collection: Vec<&str> = split_line.collect();
                    let virtual_address = usize::from_str_radix(collection[1], 16).unwrap();
                    let physical_address = usize::from_str_radix(collection[2], 16).unwrap();
                    let info = PaginationInfo {
                        physical_address: physical_address,
                        virtual_address: virtual_address,
                        size: 0x1000,
                        is_enabled: true,
                    };
                    unsafe {
                        (*PaginationL4::get_kernel_pagination()).set(info);
                    }
                } else if line_string == "pci" {
                    let devices = PciDevice::enumerate_all();
                    KernelConsole::print("Number of PCI devices: ");
                    KernelConsole::printu64dec(devices.len() as u64);
                    KernelConsole::print("\n");
                    for device in devices {
                        let info= device.get_config();
                        KernelConsole::print("Vendor ID: ");
                        KernelConsole::print(format!("{:X}", info.vendor_id).as_str());
                        KernelConsole::print(" Device ID: ");
                        KernelConsole::print(format!("{:X}", info.device_id).as_str());
                        KernelConsole::print(" Class: ");
                        KernelConsole::print(format!("{:X}", info.class_code).as_str());
                        KernelConsole::print(" Subclass: ");
                        KernelConsole::print(format!("{:X}", info.subclass).as_str());
                        KernelConsole::print(" ProgIF: ");
                        KernelConsole::print(format!("{:X}", info.prog_if).as_str());
                        KernelConsole::print("\n");
                    }
                } else if line_string == "int" {
                    InterruptDescriptorTable::init();
                } else if line_string == "ps" {
                    for process in ProcessTable::get_singleton().processes.clone(){
                        KernelConsole::print("Process: ");
                        KernelConsole::print(&*process.borrow().name);
                        KernelConsole::print("\n");
                        for thread in process.borrow().threads.clone(){
                            KernelConsole::print("    Thread: ");
                            KernelConsole::print(&*thread.borrow().name);
                            KernelConsole::print("\n");
                        }
                    }
                } else {
                    KernelConsole::print("Unknown command\n");
                }

                break;
            }
            if char != '\0' {
                KernelConsole::print_char(char);
                line[index] = char;
                index += 1;
            }
        }
    }
}
#[no_mangle]
pub fn memcpy(dest: *mut u8, src: *const u8, n: usize) {
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
#[no_mangle]
pub fn memmove(dest: *mut u8, src: *const u8, n: usize) {
    unsafe {
        if dest < src as *mut u8 {
            for i in 0..n {
                *dest.offset(i as isize) = *src.offset(i as isize);
            }
        } else {
            for i in (0..n).rev() {
                *dest.offset(i as isize) = *src.offset(i as isize);
            }
        }
    }
}