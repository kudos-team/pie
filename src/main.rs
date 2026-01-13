#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(main);
fn main(boot_info: &'static BootInfo) -> ! {
    kudos::init(boot_info, true);

    kudos::printlgln!("Hello world!");

    kudos::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kudos::test_panic_handler(info)
}
