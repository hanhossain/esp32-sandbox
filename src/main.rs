#![no_std]
#![no_main]

extern crate panic_halt;

use core::fmt::Write;
use esp32::Peripherals;
use esp32_hal::prelude::*;
use esp32_logger;

mod watchdog;

#[no_mangle]
fn main() -> ! {
    let dp = unsafe { Peripherals::steal() };

    let mut timg0 = dp.TIMG0;
    let mut timg1 = dp.TIMG1;

    watchdog::disable_main_system(&mut timg0, &mut timg1);

    esp32_logger::setup(dp.UART0, dp.RTCCNTL, dp.APB_CTRL, dp.DPORT);

    loop {
        unsafe {
            esp32_logger::log("hello world from rust!");
        }
    }
}
