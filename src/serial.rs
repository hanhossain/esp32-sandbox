use esp32::{UART0, DPORT, RTCCNTL, APB_CTRL};
use esp32_hal::clock_control::{ClockControl, XTAL_FREQUENCY_AUTO};
use esp32_hal::prelude::*;
use esp32_hal::serial::{Serial, NoTx, NoRx, Tx, Rx};
use esp32_hal::serial::config::Config;
use esp32_hal::dport::Split;

pub fn get(uart0: UART0, rtccntl: RTCCNTL, apb_ctrl: APB_CTRL, dport: DPORT) -> (Tx<UART0>, Rx<UART0>) {
    let (mut dport, dport_clock_control) = dport.split();
    let clock_control = ClockControl::new(
        rtccntl,
        apb_ctrl,
        dport_clock_control,
        XTAL_FREQUENCY_AUTO
    )
    .unwrap();

    let (clock_control_config, mut watchdog) = clock_control.freeze().unwrap();
    watchdog.disable();

    let serial = Serial::uart0(
        uart0,
        (NoTx, NoRx),
        Config::default().baudrate(115200.Hz()),
        clock_control_config,
        &mut dport
    )
    .unwrap();

    serial.split()
}