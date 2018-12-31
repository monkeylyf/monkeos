#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Only on MacOS.
#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}
