#![no_std]
#![no_main]

extern crate panic_halt;

use xtensa_lx6_rt::delay;
use esp32_hal::prelude::*;
use esp32::Peripherals;

mod watchdog;

/// The default clock source is the onboard crystal
/// In most cases 40mhz (but can be as low as 2mhz depending on the board)
const CORE_HZ: u32 = 40_000_000;

#[no_mangle]
fn main() -> ! {
    let dp = unsafe { Peripherals::steal() };

    let mut rtccntl = dp.RTCCNTL;
    let mut timg0 = dp.TIMG0;
    let mut timg1 = dp.TIMG1;

    watchdog::disable_all(&mut rtccntl, &mut timg0, &mut timg1);

    let pins = dp.GPIO.split();
    let mut led = pins.gpio2.into_open_drain_output();

    loop {
        led.set_high().unwrap();
        delay(CORE_HZ);

        led.set_low().unwrap();
        delay(CORE_HZ);
    }
}
