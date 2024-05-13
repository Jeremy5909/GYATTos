#![no_std]
#![no_main]

use core::{panic::PanicInfo, fmt::Write};

use crate::vga_buffer::WRITER;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
// Use C calling convention
pub extern "C" fn _start() -> ! {
    write!(WRITER.lock(), "Hello, World!").unwrap();

    loop {}
}