#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::i2c::{self, Error, I2c};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, peripherals};
use rtt_target::{rprintln, rtt_init_print};

const ADDRESS: u8 = 0x68;
const WHOAMI: u8 = 0x75;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    rtt_init_print!();

    //let mut i2c = I2c::new_blocking(p.I2C1, p.PA15, p.PB7, Hertz(100_000), Default::default());
    let mut i2c = I2c::new(
        p.I2C1,
        p.PA15,
        p.PB7,
        Irqs,
        NoDma,
        NoDma,
        Hertz(100_000),
        Default::default(),
    );

    let mut data = [0u8; 1];

    match i2c.blocking_write_read(ADDRESS, &[WHOAMI], &mut data) {
        Ok(()) => rprintln!("Whoami: {:#02X}", data[0]),
        Err(Error::Timeout) => rprintln!("Operation timed out"),
        Err(e) => rprintln!("I2c Error: {:?}", e),
    }
}
