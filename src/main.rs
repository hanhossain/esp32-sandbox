#![no_std]
#![no_main]

use core::panic::PanicInfo;
use esp32_hal::{
    clock_control::{sleep, ClockControl, XTAL_FREQUENCY_AUTO},
    dport::Split,
    prelude::*,
    target,
    timer::Timer,
};
use esp32_logger::*;

#[entry]
fn main() -> ! {
    let dp = target::Peripherals::take().expect("failed to acquire peripherals");
    let (mut dport, dport_clock_control) = dp.DPORT.split();

    let gpio = dp.GPIO.split();

    let clock_control = ClockControl::new(
        dp.RTCCNTL,
        dp.APB_CTRL,
        dport_clock_control,
        XTAL_FREQUENCY_AUTO,
    )
    .unwrap();

    // disable RTC watchdog
    let (clock_control_config, mut watchdog) = clock_control.freeze().unwrap();
    watchdog.disable();

    // disable MST watchdogs
    let (.., mut watchdog0) = Timer::new(dp.TIMG0, clock_control_config);
    let (.., mut watchdog1) = Timer::new(dp.TIMG1, clock_control_config);
    watchdog0.disable();
    watchdog1.disable();

    setup_logger(
        dp.UART0,
        gpio.gpio1,
        gpio.gpio3,
        clock_control_config,
        &mut dport,
    );

    let mut counter = 0;
    loop {
        log!("counter: {}", counter);
        sleep(500_000.us());
        counter += 1;
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("\r\n{:?}", info);
    loop {}
}
