#![no_std] // Ignoring the standard library 
#![no_main] // Telling the rust compiler that we are not using a normal entry point

// The vga buffer module 
mod vga_buffer;

// Importing the type for the error handler
use core::panic::PanicInfo;

/// Panic handler for the OS 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Main entry 
#[no_mangle] // No mangle == make sure the function name is _start
pub extern "C" fn _start() -> ! {
    vga_buffer::print_introduction();
    loop {}
}