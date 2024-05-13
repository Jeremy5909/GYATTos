#![no_std]
#![no_main]

use core::{panic::PanicInfo, fmt::Write};

use vga_buffer::{Buffer, Color, ColorCode, Writer};

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
// Use C calling convention
pub extern "C" fn _start() -> ! {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    writer.write_str("Hello, World").unwrap();

    loop {}
}