#![no_std]
#![no_main]

#[allow(unused_imports)]
use arduino_hal::{
    delay_ms,
    hal::{port::PB5, Pins},
    pins,
    port::{mode::Output, Pin},
    prelude::*,
    Peripherals,
};
#[allow(unused_imports)]
use halt as _;

mod halt;

fn stutter_blink(led: &mut Pin<Output, PB5>, times: usize) {
    (0..times).map(|i| i * 10).for_each(|i| {
        led.toggle();
        delay_ms(i as u16);
    });
}

#[arduino_hal::entry]
unsafe fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    let pins = pins!(peripherals);

    let mut led = pins.d13.into_output();

    loop {
        stutter_blink(&mut led, 25);
    }
}
