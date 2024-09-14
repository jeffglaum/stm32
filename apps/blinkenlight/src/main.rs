#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use hal::prelude::*;
use hal::stm32;
use stm32g4xx_hal as hal;

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("ERROR: cannot take peripherals");
    let mut rcc = dp.RCC.constrain();

    // Initialize RTT debug message interface
    rtt_init_print!();

    // Initialize LED GPIO
    rprintln!("INFO: initialize LED");
    let gpiob = dp.GPIOB.split(&mut rcc);
    let mut led = gpiob.pb8.into_push_pull_output();

    loop {
        rprintln!("INFO: set LED low");
        for _ in 0..10_000 {
            led.set_low().unwrap();
        }

        rprintln!("INFO: set LED high");
        for _ in 0..10_000 {
            led.set_high().unwrap();
        }
    }
}
