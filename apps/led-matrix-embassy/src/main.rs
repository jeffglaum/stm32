#![no_std]
#![no_main]


use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_time::Timer;
use rtt_target::{rprintln, rtt_init_print};

static MATRIX_SIZE: usize = 8;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    rtt_init_print!();

    let mut rows: [Output<'_, AnyPin>; MATRIX_SIZE] = [ 
        Output::new(p.PA0, Level::High, Speed::Low).degrade(),
        Output::new(p.PA1, Level::High, Speed::Low).degrade(),
        Output::new(p.PA2, Level::High, Speed::Low).degrade(),
        Output::new(p.PA9, Level::High, Speed::Low).degrade(),
        Output::new(p.PA4, Level::High, Speed::Low).degrade(),
        Output::new(p.PA5, Level::High, Speed::Low).degrade(),
        Output::new(p.PA6, Level::High, Speed::Low).degrade(),
        Output::new(p.PA7, Level::High, Speed::Low).degrade()];

    let mut cols: [Output<'_, AnyPin>; MATRIX_SIZE] = [ 
        Output::new(p.PA11, Level::Low, Speed::Low).degrade(),
        Output::new(p.PA10, Level::Low, Speed::Low).degrade(),
        Output::new(p.PB0, Level::Low, Speed::Low).degrade(),
        Output::new(p.PB4, Level::Low, Speed::Low).degrade(),
        Output::new(p.PB5, Level::Low, Speed::Low).degrade(),
        Output::new(p.PB6, Level::Low, Speed::Low).degrade(),
        Output::new(p.PB3, Level::Low, Speed::Low).degrade(),
        Output::new(p.PA8, Level::Low, Speed::Low).degrade(),
        ];

    let mut fb: [u8; MATRIX_SIZE] = [0; MATRIX_SIZE];

    let mut bx:i8=6;
    let mut by:i8 = 0;
    let mut dx:i8 = 1;
    let mut dy:i8 = 1;

    rprintln!("INFO: matrix size {}x{}", MATRIX_SIZE, MATRIX_SIZE);

    loop {

        fb.fill(0);
        fb[by as usize] = 0x80 >> bx;

        for (y, row) in rows.iter_mut().enumerate() {
            for (x, col) in cols.iter_mut().enumerate() {
                if fb[y] & (0x80 >> x) != 0 {
                    row.set_high();
                    col.set_low();
                }
                else {
                    row.set_low();
                    col.set_high();
                }
            }
        }

        bx = bx + dx;
        by = by + dy;

        if bx < 0 || bx > MATRIX_SIZE as i8 - 1 {
            dx = dx * -1;
            bx = bx + dx;
        }
        if by < 0 || by > MATRIX_SIZE as i8 - 2 {
            dy = dy * -1;
            by = by + dy;
        }
        Timer::after_millis(50).await;
    }
}
