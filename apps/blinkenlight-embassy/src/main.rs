#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Pin, Speed};
use embassy_time::Timer;
use rtt_target::{rprintln, rtt_init_print};

#[embassy_executor::task]
async fn blink(pin: impl Pin) {
    let mut led = Output::new(pin, Level::High, Speed::Low);

    rprintln!("INFO: blink task started");
    loop {
        led.set_high();
        Timer::after_millis(500).await;
        led.set_low();
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    rtt_init_print!();

    // Blink LED attached to PA0
    rprintln!("INFO: spawning blink task");
    spawner.spawn(blink(p.PA0)).unwrap();

    let mut led = Output::new(p.PB8, Level::High, Speed::Low);

    rprintln!("INFO: main loop");
    // Blink LED attached to PB8
    loop {
        led.set_high();
        Timer::after_millis(150).await;
        led.set_low();
        Timer::after_millis(150).await;
    }
}
