use crate::cpu_ports::{port_input8, port_output8};

pub fn print(text: &str) {
    let text_pointer = text.as_ptr();
    let count = text.len();
    let mut i = 0;
    loop {
        unsafe {
            print_char(*(text_pointer.offset(i)) as char);
            i = i + 1;
            if i == count as isize {
                break;
            }
        }
    }
}

pub fn print_char(char: char) {
    unsafe {
        let mut position = get_cursor_position();
        let addres = 0xb8000 + 2 * position as u32;
        *(addres as *mut u8) = char as u8;
        *((addres + 1) as *mut u8) = 0x7;

        position += 1;

        set_cursor_position(position)
    }
}

pub fn get_cursor_position() -> u16 {
    let mut result: u16 = 0;
    port_output8(0x3d4, 0x0f);
    result |= port_input8(0x3d5) as u16;
    port_output8(0x3d4, 0x0e);
    result |= (port_input8(0x3d5) as u16) << 8;
    return result;
}

pub fn set_cursor_position(position: u16) {
    port_output8(0x3d4, 0x0f);
    port_output8(0x3d5, (position & 0xff) as u8);
    port_output8(0x3d4, 0x0e);
    port_output8(0x3d5, ((position >> 8) & 0xff) as u8);
}