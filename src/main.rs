#![no_std]
#![no_main] // Don't use the normal entry point
mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// static START: &[u8] = b"Starting!";

#[no_mangle]
// Overwrite OS entry point
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}