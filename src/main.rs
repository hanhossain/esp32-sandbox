#![no_std]
#![no_main]

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

    let (mut clock_control_config, ..) = setup_logger!(dp);

    clock_control_config.start_core(1, run_core1).unwrap();

    loop {
        sleep(1.s());
        log!("Core 0");
    }
}

fn run_core1() -> ! {
    loop {
        sleep(3.s());
        log!("Core 1");
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{:?}", info);
    loop {}
}
