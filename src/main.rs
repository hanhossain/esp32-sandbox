#![no_std]
#![no_main]

use core::panic::PanicInfo;
use esp32_hal::{
    clock_control::{sleep, ClockControl, XTAL_FREQUENCY_AUTO},
    dport::Split,
    gpio::{Event, Gpio0, Input, Pin, PullUp},
    interrupt::Interrupt,
    prelude::*,
    target,
    timer::Timer,
};

static BUTTON: CriticalSectionSpinLockMutex<Option<Gpio0<Input<PullUp>>>> =
    CriticalSectionSpinLockMutex::new(None);

static BTN_COUNTER: CriticalSectionSpinLockMutex<usize> = CriticalSectionSpinLockMutex::new(0);

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

    esp32_logger::init(dp.UART0, gpios.gpio1, gpios.gpio3, clock_control_config);

    let mut button = gpios.gpio0.into_pull_up_input();
    button.listen(Event::FallingEdge);

    (&BUTTON).lock(|x| *x = Some(button));

    interrupt::enable(Interrupt::GPIO_INTR).unwrap();

    let mut counter = 0;

    loop {
        counter += 1;
        esp32_logger::log!("counter: {}", counter);
        sleep(500.ms());
    }
}

#[interrupt]
fn GPIO_INTR() {
    (&BUTTON, &BTN_COUNTER).lock(|button, counter| {
        let button = button.as_mut().unwrap();
        *counter += 1;

        if button.is_interrupt_set() {
            esp32_logger::log!("button pressed {} times!", counter);
            button.clear_interrupt();
        }
    });
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    esp32_logger::error!("\r\n{:?}", info);
    loop {}
}
