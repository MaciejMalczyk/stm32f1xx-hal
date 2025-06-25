#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m::asm;

use stm32f1xx_hal::{
    pac,
    prelude::*,
    time::Hz,
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();

    // setting registers to enter stop mode when no interrupt

    dp.RCC.cr().modify(|_r, w| {
        w.pllon().on() // pll on
    });

    dp.RCC.cfgr().modify(|_r, w| {
        w.pllmul().mul8(); // set pll multiplier
        w.sw().pll() // set pll as system clock
    });

    dp.RCC.apb2enr().modify(|_r, w| {
        w.iopaen().enabled() // I/O port A clock enable
    });

    // deep sleep init

    dp.PWR.cr().modify(|_r, w| {
        w.pdds().stop_mode(); // enter stop mode when enters deep sleep
        w.lpds().set_bit() // voltage regulator to low power mode when stop
    });

    cp.SCB.set_sleepdeep(); // enter sleep mode

    dp.RCC.csr().modify(|_r, w| {
        w.lsion().on() // use internal low frequency oscilator
    });

    // wait until lsion is on
    while dp.RCC.csr().read().lsion().is_off() {
        asm::nop();
    }

    // configuring and freezing clocks
    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let _clocks = rcc
        .cfgr
        .use_hse(Hz(8_000_000))
        .freeze(&mut flash.acr);


    loop {
        asm::wfi();
    }
}
