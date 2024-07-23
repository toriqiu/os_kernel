#![no_std]
#![no_main] // Don't use the normal entry point chain

use core::panic::PanicInfo;

// PanicInfo contains file and line where the panic happened 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {}
