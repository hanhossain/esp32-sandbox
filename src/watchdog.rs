//! A bunch of watchdog helper methods

use esp32::{RTCCNTL, TIMG0, TIMG1};

const PROTECT_RESET_VALUE: u32 = 0x50D83AA1;

pub fn disable(rtccntl: &mut RTCCNTL, timg0: &mut TIMG0, timg1: &mut TIMG1) {
    disable_rwdt(rtccntl);
    disable_mwdts(timg0, timg1);
}

/// Disables the RTC Watchdog Timer (RWDT)
fn disable_rwdt(rtccntl: &mut RTCCNTL) {
    rtccntl.wdtwprotect.write(|w| unsafe { w.bits(PROTECT_RESET_VALUE) });
    rtccntl.wdtconfig0.modify(|_, w| unsafe {
        w
            .wdt_stg0()
            .bits(0x0)
            .wdt_stg1()
            .bits(0x0)
            .wdt_stg2()
            .bits(0x0)
            .wdt_stg3()
            .bits(0x0)
            .wdt_flashboot_mod_en()
            .clear_bit()
            .wdt_en()
            .clear_bit()
    });
    rtccntl.wdtwprotect.write(|w| unsafe { w.bits(0x0) });
}

/// Disables the Main System Watchdog Timers (MWDT)
fn disable_mwdts(timg0: &mut TIMG0, timg1: &mut TIMG1) {
    // disable write protection
    timg0
        .wdtwprotect
        .write(|w| unsafe { w.bits(PROTECT_RESET_VALUE) });
    timg1
        .wdtwprotect
        .write(|w| unsafe { w.bits(PROTECT_RESET_VALUE) });

    // disable watchdogs
    timg0
        .wdtconfig0
        .write(|w| unsafe { w.bits(0x0) });
    timg1
        .wdtconfig0
        .write(|w| unsafe { w.bits(0x0) });
}