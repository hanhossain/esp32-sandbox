#![no_std]
#![no_main]

use core::fmt::Write;
use esp32::Peripherals;
use core::panic::PanicInfo;
use esp32_hal::prelude::*;
use esp32_logger::{log, STORED_TX};
use esp32_hal::clock_control::sleep;

mod watchdog;

#[no_mangle]
fn main() -> ! {
    let dp = unsafe { Peripherals::steal() };

    let mut timg0 = dp.TIMG0;
    let mut timg1 = dp.TIMG1;

    watchdog::disable_main_system(&mut timg0, &mut timg1);

    esp32_logger::setup(dp.UART0, dp.RTCCNTL, dp.APB_CTRL, dp.DPORT);
    log!("Hello world from rust!");

    // setup led
    let pins = dp.GPIO.split();
    let mut led = pins.gpio2.into_open_drain_output();

    let mut counter = 0;

    loop {
        log!("counter: {}", counter);
        counter += 1;

        led.set_high().unwrap();
        sleep(800.ms());
        led.set_low().unwrap();
        sleep(200.ms());
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log!("{:?}", info);
    loop {}
}
