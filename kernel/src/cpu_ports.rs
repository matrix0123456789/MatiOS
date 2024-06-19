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