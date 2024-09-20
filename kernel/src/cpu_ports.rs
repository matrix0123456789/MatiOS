pub fn port_input8(port: u16) -> u8 {
    let result: u8;
    unsafe {
        core::arch::asm!(
        "in al, dx",
        in("dx") port,
        out("al") result,
        );
    }
    return result;
}
pub fn port_output8(port: u16, value: u8) {
    unsafe {
        core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        );
    }
}
pub fn port_input32(port: u16) -> u32 {
    let result: u32;
    unsafe {
        core::arch::asm!(
        "in eax, dx",
        in("dx") port,
        out("eax") result,
        );
    }
    return result;
}
pub fn port_output32(port: u16, value: u32) {
    unsafe {
        core::arch::asm!(
        "out dx, eax",
        in("dx") port,
        in("eax") value,
        );
    }
}