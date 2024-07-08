use crate::cpu_ports::{port_input8, port_output8};
pub struct KernelConsole {}

static mut last_keyboard_code: u8 = 0;
static mut COLOR: u8 = 0x7;
impl KernelConsole {
    pub fn print(text: &str) {
        let text_pointer = text.as_ptr();
        let count = text.len();
        let mut i = 0;
        loop {
            unsafe {
                KernelConsole::print_char(*(text_pointer.offset(i)) as char);
                i = i + 1;
                if i == count as isize {
                    break;
                }
            }
        }
    }

    pub fn print_char(char: char) {
        unsafe {
            let mut position = KernelConsole::get_cursor_position();
            if char == '\n' {
                position = position + 80 - position % 80;
            } else {
                let addres = 0xb8000 + 2 * position as u32;
                *(addres as *mut u8) = char as u8;
                *((addres + 1) as *mut u8) = COLOR;

                position += 1;
            }
            if position >= 25 * 80 {
                for i in 80..25 * 80 {
                    let addres = 0xb8000 + 2 * i as u32;
                    let addres2 = 0xb8000 + 2 * (i - 80) as u32;
                    *(addres2 as *mut u16) = *(addres as *mut u16);
                }
                position -= 80;
                for i in 24*80..25*80{
                    let addres = 0xb8000 + 2 * i as u32;
                    *(addres as *mut u8) = 0;
                    *((addres + 1) as *mut u8) = 0x7;
                }
            }

            KernelConsole::set_cursor_position(position)
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
    pub fn printu64hex(mut value: u64) {
        KernelConsole::print("0x");
        for _ in 0..16 {
            let mut current = (value >> 60) as u8;
            value <<= 4;
            if current < 10 {
                current += 48;
            } else {
                current += 55;
            }
            KernelConsole::print_char(current as char);
        }
    }
    pub fn printu64dec(mut value: u64) {
        let mut buffer = [0u8; 20];
        let mut i = 0;
        loop {
            buffer[i] = (value % 10) as u8 + 48;
            value /= 10;
            i += 1;
            if value == 0 {
                break;
            }
        }
        while i > 0 {
            i -= 1;
            KernelConsole::print_char(buffer[i] as char);
        }
    }
    pub fn read_and_wait() -> char {
        loop {
            unsafe {
                let mut code = port_input8(0x60);
                if code != last_keyboard_code {
                    last_keyboard_code = code;
                    return Self::convert_key_code_to_ascii(code);
                }
            }
        }
    }
    pub fn convert_key_code_to_ascii(code: u8) -> char {
        return match code {
            0x1e => 'a',
            0x30 => 'b',
            0x2e => 'c',
            0x20 => 'd',
            0x12 => 'e',
            0x21 => 'f',
            0x22 => 'g',
            0x23 => 'h',
            0x17 => 'i',
            0x24 => 'j',
            0x25 => 'k',
            0x26 => 'l',
            0x32 => 'm',
            0x31 => 'n',
            0x18 => 'o',
            0x19 => 'p',
            0x10 => 'q',
            0x13 => 'r',
            0x1f => 's',
            0x14 => 't',
            0x16 => 'u',
            0x2f => 'v',
            0x11 => 'w',
            0x2d => 'x',
            0x15 => 'y',
            0x2c => 'z',
            0x02 => '1',
            0x03 => '2',
            0x04 => '3',
            0x05 => '4',
            0x06 => '5',
            0x07 => '6',
            0x08 => '7',
            0x09 => '8',
            0x0a => '9',
            0x0b => '0',
            0x1c => '\n',
            0x39 => ' ',
            _ => '\0',
        }
    }
    pub fn set_color(color: u8) {
        unsafe {
            COLOR = color;
        }
    }
    pub fn clear_screen() {
        unsafe {
            for i in 0..80 * 25 {
                let addres = 0xb8000 + 2 * i as u32;
                *(addres as *mut u8) = 0;
                *((addres + 1) as *mut u8) = COLOR;
            }
        }
        KernelConsole::set_cursor_position(0);
    }
}