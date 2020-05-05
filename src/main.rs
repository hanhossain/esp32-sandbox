#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;
use esp32::Peripherals;
use esp32_hal::clock_control::sleep;
use esp32_hal::prelude::*;
use esp32_logger::*;

mod watchdog;

#[no_mangle]
fn main() -> ! {
    let dp = unsafe { Peripherals::steal() };

    let mut timg0 = dp.TIMG0;
    let mut timg1 = dp.TIMG1;

    watchdog::disable_main_system(&mut timg0, &mut timg1);

    setup_logger!(dp);
    log!("Hello world from rust!");

    let mut counter = 0;

    loop {
        log!("counter: {}", counter);
        counter += 1;

        sleep(1.s());
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log!("{:?}", info);
    loop {}
}
