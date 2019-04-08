#![no_std]  // No link to the Rust standard library.
#![no_main]  // Disabl all Rust-level entry points

use core::panic::PanicInfo;

// The following function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello world!";

// Only on MacOS.
// Do not mangle the name of following function.
#[no_mangle]
// Entry point function.
// Linker looks for a function named "_start" by default.
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
