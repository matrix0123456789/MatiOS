#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {

    loop {}
}
#[no_mangle]
pub extern "C" fn _start() {
    unsafe {
        *(0xb8000 as *mut u8)='R' as u8;
        *(0xb8001 as *mut u8)=0x4;
        *(0xb8002 as *mut u8)='U' as u8;
        *(0xb8003 as *mut u8)=0x4;
        *(0xb8004 as *mut u8)='S' as u8;
        *(0xb8005 as *mut u8)=0x4;
        *(0xb8006 as *mut u8)='T' as u8;
        *(0xb8007 as *mut u8)=0x4;
    }

}
