#![no_std]
#![no_main] // Don't use the normal entry point
#![feature(custom_test_frameworks)]
#![test_runner(os_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os_kernel::println;


// Normal run
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os_kernel::hlt_loop(); 
}

// Test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_kernel::test_panic_handler(info)
}


#[no_mangle]
// Overwrite OS entry point
pub extern "C" fn _start() -> ! {
    println!("Starting{}", "!");

    os_kernel::init(); 
    
    // Conditional compilation to avoid testing on normal runs
    #[cfg(test)]
    test_main();

    println!("No crash");
    os_kernel::hlt_loop(); 
    loop {}
}


// ------------------ TESTING ------------------
// #[cfg(test)]
// pub fn test_runner(tests: &[&dyn Testable]) { 
//     // Print to serial interface instead of VGA buffer
//     serial_println!("Running {} tests", tests.len());
//     for test in tests {
//         test.run();
//     }
//     exit_qemu(QemuExitCode::Success);
// }

// `tests` slice passed to test_runner contains reference to test functions 
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

