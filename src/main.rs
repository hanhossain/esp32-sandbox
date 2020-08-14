#![no_std]
#![no_main]

use core::panic::PanicInfo;
use esp32_hal::{
    clock_control::{ClockControl, XTAL_FREQUENCY_AUTO},
    dport::Split,
    dprint,
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

    esp32_logger::setup(
        dp.UART0,
        gpio.gpio1,
        gpio.gpio3,
        clock_control_config,
        &mut dport,
    );

    dprint!("this is from dprint\r\n");

    log!("hello from logger");
    warn!("this is a warning");
    error!("something broke");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    dprint!("\r\n{:?}\r\n", info);
    loop {}
}
