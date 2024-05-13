#![no_std]
#![no_main]

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[no_mangle]
// Use C calling convention
pub extern "C" fn _start() -> ! {
    loop {}
}