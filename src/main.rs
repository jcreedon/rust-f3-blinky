#![no_std]
#![deny(unsafe_code)]
#![deny(warnings)]

extern crate cortex_m;
extern crate f3;
extern crate panic_abort;

use f3::hal::stm32f30x;
use f3::hal::prelude::*;
use f3::hal::delay::Delay;
use f3::led::Leds;

fn main() {
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let stm_peripherals = stm32f30x::Peripherals::take().unwrap();

    let mut flash = stm_peripherals.FLASH.constrain();
    let mut rcc = stm_peripherals.RCC.constrain();
    let gpioe = stm_peripherals.GPIOE.split(&mut rcc.ahb);

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut leds = Leds::new(gpioe);
    let mut delay = Delay::new(cortex_peripherals.SYST, clocks);

    loop {
        leds[0].on();
        delay.delay_ms(500_u16);
        leds[0].off();
        delay.delay_ms(500_u16);
    }

}
