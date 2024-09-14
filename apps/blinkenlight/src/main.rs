#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::entry;
extern crate stm32g4xx_hal as hal;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    // Initialze RTT debug message interface
    rtt_init_print!();

    loop {
        rprintln!("Hello World!");
    }
}
