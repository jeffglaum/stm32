#![no_std]
#![no_main]

use panic_halt as _;

use core::cell::RefCell;
use cortex_m_rt::entry;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::spi::{BitOrder, Config, Spi, MODE_0};
use embassy_stm32::time::Hertz;
use embassy_sync::blocking_mutex::NoopMutex;
use is31fl3743b_driver::{CSy, Is31fl3743b, SWx};
use rtt_target::{rprintln, rtt_init_print};
use static_cell::StaticCell;
#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());

    rtt_init_print!();
    rprintln!("Lumissil IS31FL3743B");

    let mut spi_config = Config::default();
    spi_config.mode = MODE_0;
    spi_config.bit_order = BitOrder::MsbFirst;
    spi_config.frequency = Hertz(1_000_000);

    static SPI_BUS: StaticCell<
        NoopMutex<RefCell<Spi<'_, embassy_stm32::peripherals::SPI1, NoDma, NoDma>>>,
    > = StaticCell::new();

    let spi = Spi::new(p.SPI1, p.PB3, p.PB5, p.PB4, NoDma, NoDma, spi_config);
    let spi_bus = NoopMutex::new(RefCell::new(spi));
    let spi_bus = SPI_BUS.init(spi_bus);

    let cs = Output::new(p.PA11, Level::High, Speed::VeryHigh);

    let spi_dev = SpiDevice::new(spi_bus, cs);

    let mut driver = Is31fl3743b::new(spi_dev).unwrap();

    // Perform some configuration
    let _ = driver.enable_phase_delay();
    let _ = driver.set_global_current(90);

    // Adjust current/brightness of specific LED (by given SWx and CSy coordinates)
    let _ = driver.set_led_peak_current(SWx::SW2, CSy::CS10, 50);
    let _ = driver.set_led_brightness(SWx::SW2, CSy::CS10, 50);

    loop {}
}
