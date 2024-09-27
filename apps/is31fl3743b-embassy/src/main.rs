#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use core::convert::TryFrom;
use defmt::info;
//use defmt::{info, panic};
use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::rng::Rng;
use embassy_stm32::spi::{BitOrder, Config, Spi, MODE_0};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, peripherals, rng};
//use embassy_time::Delay;
use embedded_hal_bus::spi::ExclusiveDevice;
use is31fl3743b_driver::{CSy, Is31fl3743b, SWx};

bind_interrupts!(struct Irqs {
    RNG => rng::InterruptHandler<peripherals::RNG>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    info!("Lumissil IS31FL3743B");

    // Configure spi bus and gpio for chip select control
    let mut spi_config = Config::default();
    spi_config.mode = MODE_0;
    spi_config.bit_order = BitOrder::MsbFirst;
    spi_config.frequency = Hertz(10_000_000);

    let spi = Spi::new(p.SPI1, p.PB3, p.PB5, p.PB4, NoDma, NoDma, spi_config);
    let cs = Output::new(p.PA11, Level::High, Speed::VeryHigh);

    // Get access to LED matrix controller on the spi bus
    let spi_dev = ExclusiveDevice::new_no_delay(spi, cs).unwrap();
    let mut driver = Is31fl3743b::new(spi_dev).unwrap();

    // TODO - short test is falsely indicating shorts.
    // Perform a test which detects shorted LEDs
    //let delay = Delay;
    //let led_status = driver.detect_shorts(delay).unwrap();
    //if !led_status.all_leds_ok() {
    //panic!("There is a short amongst the LEDs");
    //}

    // Global configuration
    let _ = driver.enable_phase_delay();
    let _ = driver.set_global_current(90);

    // Random number generator
    let mut rng = Rng::new(p.RNG, Irqs);

    loop {
        // Randomize column, row, and intensity (on-off)
        let mut pos = [0u8; 1];
        rng.async_fill_bytes(&mut pos).await.unwrap();
        let x = (pos[0] % 11) + 1;
        let y = (pos[0] % 18) + 1;
        let mut int = [0u8; 1];
        rng.async_fill_bytes(&mut int).await.unwrap();
        let i = if int[0] % 2 == 0 { 25 } else { 0 };

        // Adjust current/brightness of specific LED (by given SWx and CSy coordinates)
        let _ =
            driver.set_led_peak_current(SWx::try_from(x).unwrap(), CSy::try_from(y).unwrap(), i);
        let _ = driver.set_led_brightness(SWx::try_from(x).unwrap(), CSy::try_from(y).unwrap(), i);
    }
}
