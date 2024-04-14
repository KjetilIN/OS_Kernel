#![no_std] // Ignoring the standard library 
#![no_main] // Telling the rust compiler that we are not using a normal entry point

mod vga_buffer;

// Importing the type for the error handler
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// The Hello text as a list of bytes 
static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // No mangle == make sure the function name is _start
pub extern "C" fn _start() -> ! {
    // address of the VGA buffer
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // Set the corresponding ascii byte
            *vga_buffer.offset(i as isize * 2) = byte;

            // Set the color byte, in this case cyan
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}