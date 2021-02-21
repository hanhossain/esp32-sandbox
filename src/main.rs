#![no_std]
#![no_main]

use core::panic::PanicInfo;
use esp32_hal::{
    clock_control::{sleep, ClockControl, XTAL_FREQUENCY_AUTO},
    dport::Split,
    prelude::*,
    target,
    timer::Timer,
    {dprint, dprintln},
};

#[entry]
fn main() -> ! {
    let dp = target::Peripherals::take().expect("failed to acquire peripherals");
    let (_, dport_clock_control) = dp.DPORT.split();

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

    let gpios = dp.GPIO.split();
    let mut led = gpios.gpio25.into_push_pull_output();

    let mut counter = 0;

    loop {
        counter += 1;

        dprintln!("LED On - {}", counter);
        led.set_high().unwrap();
        sleep(500.ms());

        dprintln!("LED Off - {}", counter);
        led.set_low().unwrap();
        sleep(500.ms());

        if counter > 5 {
            assert_eq!(3, 4);
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    dprintln!("{:?}", info);
    loop {}
}
