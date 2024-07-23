#![no_std]
#![no_main] // Don't use the normal entry point
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


mod vga_buffer;
mod serial;


use core::panic::PanicInfo;

// Panic handler for normal run
#[cfg(not(test))] 
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Panic handler for test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[no_mangle]
// Overwrite OS entry point
pub extern "C" fn _start() -> ! {
    println!("Starting{}", "!");

    // Conditional compilation to avoid testing on normal runs
    #[cfg(test)]
    test_main();
    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32); // iosize is 4 bytes
    }
}


// ------------------ TESTING ------------------
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    // Print to serial interface instead of VGA buffer
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

// `tests` slice passed to test_runner contains reference to test functions 
#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(0, 1);
    println!("[ok]");
}

