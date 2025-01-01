#![no_std]
#![no_main]

// Reads from a rotary encoder and advances a stepper moter (unipolar, 6-wire) in the direction selected.

#[allow(unused_imports)]
use num_traits::real::Real;

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pull, Speed};
use embassy_time::Timer;
use rtt_target::{rprintln, rtt_init_print};

async fn drive(s: &mut [Output<'_, AnyPin>; 4], state: u8, delay: u64) {
    match state {
    0 => {
        s[0].set_high();
        s[1].set_high();
        s[2].set_low();
        s[3].set_low();
    },
    1 => {
        s[0].set_low();
        s[1].set_high();
        s[2].set_high();
        s[3].set_low();
    },
    2 => {
        s[0].set_low();
        s[1].set_low();
        s[2].set_high();
        s[3].set_high();
    },
    3 => {
        s[0].set_high();
        s[1].set_low();
        s[2].set_low();
        s[3].set_high();
    },
    _ => {}
    }

    Timer::after_micros(delay).await;

}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    rtt_init_print!();

    let mut s: [Output<'_, AnyPin>; 4] = [ 
        Output::new(p.PA0, Level::Low, Speed::Low).degrade(),
        Output::new(p.PA1, Level::Low, Speed::Low).degrade(),
        Output::new(p.PA5, Level::Low, Speed::Low).degrade(),
        Output::new(p.PA4, Level::Low, Speed::Low).degrade()];

    let clk = Input::new(p.PA2, Pull::None);
    let dt = Input::new(p.PA7, Pull::None);

    rprintln!("INFO: running...");

    let mut lclk = clk.get_level();
    let mut state: i8 = 0;
    loop {
        if clk.get_level() != lclk {
            lclk = clk.get_level();
            let ldt = dt.get_level();
            if lclk == Level::High && ldt == Level::Low {
                state = state - 1;
                if state < 0 {
                    state = 3;
                }
                drive(&mut s, state as u8, 1500).await;
            }
            else if lclk == Level::High && ldt == Level::High {
                state = state + 1;
                if state > 3 {
                    state = 0;
                }
                drive(&mut s, state as u8, 1500).await;
            }
            else if lclk == Level::Low && ldt == Level::High {
                state = state - 1;
                if state < 0 {
                    state = 3;
                }
                drive(&mut s, state as u8, 1500).await;
            }
            else if lclk == Level::Low && ldt == Level::Low {
                state = state + 1;
                if state > 3 {
                    state = 0;
                }
                drive(&mut s, state as u8, 1500).await;
            }
        }
    }
}
