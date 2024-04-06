#![no_std] // Ignoring the standard library 
#![no_main] // Telling the rust compiler that we are not using a normal entry point

// Importing the type for the error handler
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[no_mangle] // No mangle == make sure the function name is _start
pub extern "C" fn _start() -> ! {
    loop {}
}