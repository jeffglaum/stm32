#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::spi::{BitOrder, Config, Spi, MODE_0};
use embassy_stm32::time::Hertz;
use rtt_target::{rprintln, rtt_init_print};

const ADDR_WRITE_PAGE0: u8 = 0x50;
const ADDR_WRITE_PAGE1: u8 = 0x51;
const ADDR_WRITE_PAGE2: u8 = 0x52;

static PWM_GAMMA64: [u8; 64] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0b, 0x0d, 0x0f, 0x11, 0x13, 0x16,
    0x1a, 0x1c, 0x1d, 0x1f, 0x22, 0x25, 0x28, 0x2e, 0x34, 0x38, 0x3c, 0x40, 0x44, 0x48, 0x4b, 0x4f,
    0x55, 0x5a, 0x5f, 0x64, 0x69, 0x6d, 0x72, 0x77, 0x7d, 0x80, 0x88, 0x8d, 0x94, 0x9a, 0xa0, 0xa7,
    0xac, 0xb0, 0xb9, 0xbf, 0xc6, 0xcb, 0xcf, 0xd6, 0xe1, 0xe9, 0xed, 0xf1, 0xf6, 0xfa, 0xfe, 0xff,
];

macro_rules! lumissil_write {
    ($spi:ident, $cs:ident, $da:expr, $ra:expr, $rd:expr) => {
        $cs.set_low();
        let _ = $spi.blocking_write($da);
        let _ = $spi.blocking_write($ra);
        let _ = $spi.blocking_write($rd);
        $cs.set_high();
    };
}
#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());

    rtt_init_print!();
    rprintln!("Lumissil IS31FL3743B test");

    let mut spi_config = Config::default();
    spi_config.mode = MODE_0;
    spi_config.bit_order = BitOrder::MsbFirst;
    spi_config.frequency = Hertz(1_000_000);

    let mut spi = Spi::new(p.SPI1, p.PB3, p.PB5, p.PB4, NoDma, NoDma, spi_config);
    let mut cs = Output::new(p.PA11, Level::High, Speed::VeryHigh);

    // Initialize Lumissil
    // PWM
    for n in 0u8..0xC7u8 {
        lumissil_write!(spi, cs, &[ADDR_WRITE_PAGE0], &[n], &[0u8]);
    }
    // Scaling
    for n in 1u8..0xC7u8 {
        lumissil_write!(spi, cs, &[ADDR_WRITE_PAGE1], &[n], &[0xFFu8]);
    }
    // GCC
    lumissil_write!(spi, cs, &[ADDR_WRITE_PAGE2], &[0x02u8], &[0x70u8]);
    lumissil_write!(spi, cs, &[ADDR_WRITE_PAGE2], &[0x01u8], &[0xFFu8]);
    lumissil_write!(spi, cs, &[ADDR_WRITE_PAGE2], &[0x00u8], &[0x09u8]);

    // Loop through colors - all LEDs
    loop {
        // Blue
        for j in 0u8..64u8 {
            for i in (1u8..0xC7u8).step_by(3) {
                let gamma = PWM_GAMMA64[j as usize];
                lumissil_write!(spi, cs, &[ADDR_WRITE_PAGE0], &[i], &[gamma]);
            }
        }
        // Green
        for j in 0u8..64u8 {
            for i in (2u8..0xC7u8).step_by(3) {
                let gamma = PWM_GAMMA64[j as usize];
                lumissil_write!(spi, cs, &[ADDR_WRITE_PAGE0], &[i], &[gamma]);
            }
        }
        // Red
        for j in 0u8..64u8 {
            for i in (3u8..0xC7u8).step_by(3) {
                let gamma = PWM_GAMMA64[j as usize];
                lumissil_write!(spi, cs, &[ADDR_WRITE_PAGE0], &[i], &[gamma]);
            }
        }
    }
}
