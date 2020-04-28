#![no_std]
#![no_main]

extern crate panic_halt;

use core::fmt::Write;
use esp32::Peripherals;
// use esp32_hal::clock_control::sleep;

use esp32_hal::prelude::*;

mod serial;
mod watchdog;

// const BLINK_HZ: Hertz = Hertz(2);

#[no_mangle]
fn main() -> ! {
    let dp = unsafe { Peripherals::steal() };

    let mut timg0 = dp.TIMG0;
    let mut timg1 = dp.TIMG1;

    watchdog::disable_main_system(&mut timg0, &mut timg1);

    // let pins = dp.GPIO.split();
    // let mut led = pins.gpio2.into_open_drain_output();

    serial::setup(
        dp.UART0,
        dp.RTCCNTL,
        dp.APB_CTRL,
        dp.DPORT
    );

    loop {
        serial::log("hello world");

        // led.set_high().unwrap();
        // sleep((Hertz(1_000_000) / BLINK_HZ).us());

        // led.set_low().unwrap();
        // sleep((Hertz(1_000_000) / BLINK_HZ).us());
    }
}
