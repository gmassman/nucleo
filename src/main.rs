#![deny(unsafe_code)]
#![no_std]
#![no_main]

// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32l4xx_hal as hal;
use stm32l4xx_hal::prelude::*;

#[entry]
fn main() -> ! {
    hprintln!("Starting to blink!").unwrap();
    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let device_peripherals = hal::stm32::Peripherals::take().unwrap();

    let mut flash = device_peripherals.FLASH.constrain();
    let mut rcc = device_peripherals.RCC.constrain();
    let mut pwr = device_peripherals.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc.cfgr.hclk(4.MHz()).freeze(&mut flash.acr, &mut pwr);

    let mut gpiob = device_peripherals.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob.pb3.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut timer = hal::delay::Delay::new(core_peripherals.SYST, clocks);

    loop {
        timer.delay_ms(1000_u32);
        led.set_high();

        timer.delay_ms(1000_u32);
        led.set_low();
    }
}
