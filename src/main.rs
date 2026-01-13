#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kudos::print!("Hello world!");
    kudos::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kudos::test_panic_handler(info)
}
