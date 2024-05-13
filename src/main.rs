#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
// Use C calling convention
pub extern "C" fn _start() -> ! {
    println!("Hello, World!");
    panic!("Uh oh, nothing went wrong!");
}