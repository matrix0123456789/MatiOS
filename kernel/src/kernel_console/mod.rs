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
        if(char == '\n') {
            position = position + 80 - position % 80;
        }
        else {
            let addres = 0xb8000 + 2 * position as u32;
            *(addres as *mut u8) = char as u8;
            *((addres + 1) as *mut u8) = 0x7;

            position += 1;
        }
        if(position>=25*80){
            for i in 80..25*80{
                let addres = 0xb8000 + 2 * i as u32;
                let addres2 = 0xb8000 + 2 * (i-80) as u32;
                *(addres2 as *mut u16) = *(addres as *mut u16);
            }
            position-=80;
        }

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