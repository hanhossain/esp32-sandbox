//! A bunch of watchdog helper methods

use esp32::{rtccntl, timg, RTCCNTL, TIMG0, TIMG1};

const PROTECT_RESET_VALUE: u32 = 0x50D83AA1;

#[allow(dead_code)]
pub fn disable_all(rtccntl: &mut RTCCNTL, timg0: &mut TIMG0, timg1: &mut TIMG1) {
    disable_rtc(rtccntl);
    disable_main_system(timg0, timg1);
}

#[allow(dead_code)]
/// Disables the RTC Watchdog Timer (RWDT)
pub fn disable_rtc(rtccntl: &mut RTCCNTL) {
    let wp = WriteProtection::RTC(&rtccntl.wdtwprotect);
    wp.disable();

    rtccntl.wdtconfig0.modify(|_, w| unsafe {
        w.wdt_stg0().bits(0x0);
        w.wdt_stg1().bits(0x0);
        w.wdt_stg2().bits(0x0);
        w.wdt_stg3().bits(0x0);
        w.wdt_flashboot_mod_en().clear_bit();
        w.wdt_en().clear_bit()
    });

    wp.enable();
}

/// Disables the Main System Watchdog Timers (MWDT)
pub fn disable_main_system(timg0: &mut TIMG0, timg1: &mut TIMG1) {
    let wp0 = WriteProtection::MainSystem(&timg0.wdtwprotect);
    let wp1 = WriteProtection::MainSystem(&timg1.wdtwprotect);

    // disable write protection
    wp0.disable();
    wp1.disable();

    // disable watchdogs
    timg0.wdtconfig0.write(|w| unsafe { w.bits(0x0) });
    timg1.wdtconfig0.write(|w| unsafe { w.bits(0x0) });

    // enable write protection
    wp0.enable();
    wp1.enable();
}

enum WriteProtection<'a> {
    RTC(&'a rtccntl::WDTWPROTECT),
    MainSystem(&'a timg::WDTWPROTECT),
}

impl<'a> WriteProtection<'a> {
    fn disable(&self) {
        match self {
            WriteProtection::RTC(reg) => reg.write(|w| unsafe { w.bits(PROTECT_RESET_VALUE) }),
            WriteProtection::MainSystem(reg) => {
                reg.write(|w| unsafe { w.bits(PROTECT_RESET_VALUE) })
            }
        }
    }

    fn enable(&self) {
        match self {
            WriteProtection::RTC(reg) => reg.write(|w| unsafe { w.bits(0x0) }),
            WriteProtection::MainSystem(reg) => reg.write(|w| unsafe { w.bits(0x0) }),
        }
    }
}
