#![no_std]
#![no_main]

// Reads from a rotary encoder and advances a linear motor (Toshiba TB6612FNG controller) in the direction selected.

#[allow(unused_imports)]
use num_traits::real::Real;

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use embassy_executor::Spawner;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::time::khz;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::Channel;
use embassy_stm32::gpio::{Input, Level, Pull, Output, Speed};
use rtt_target::{rprintln, rtt_init_print};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    rtt_init_print!();

    let pwm_pin = PwmPin::new_ch1(p.PA11, OutputType::PushPull);
    let mut pwm = SimplePwm::new(p.TIM4, Some(pwm_pin), None, None, None, khz(100), Default::default());
    pwm.enable(Channel::Ch1);

    let max_duty = pwm.get_max_duty();

    rprintln!("INFO: PWM initialized");
    rprintln!("INFO: PWM max duty {}", max_duty);

    pwm.set_duty(Channel::Ch1, 0);

    let clk = Input::new(p.PA2, Pull::None);
    let dt = Input::new(p.PA7, Pull::None);

    let mut an1 = Output::new(p.PA0, Level::Low, Speed::Low).degrade();
    let mut an2 = Output::new(p.PA1, Level::Low, Speed::Low).degrade();

    let mut lclk = clk.get_level();
    let mut speed: i16 = 0;

    loop {
        if clk.get_level() != lclk {
            lclk = clk.get_level();
            let ldt = dt.get_level();
            if lclk == Level::High && ldt == Level::Low {
                speed = speed - 5;
                if speed < -1 * (max_duty as i16) {
                    speed = -1 * (max_duty as i16);
                }
                rprintln!("INFO: PWM duty {}", speed);
                pwm.set_duty(Channel::Ch1, speed.abs() as u16);
            }
            else if lclk == Level::High && ldt == Level::High {
                speed = speed + 5;
                if speed > max_duty as i16 {
                    speed = max_duty as i16;
                }
                rprintln!("INFO: PWM duty {}", speed);
                pwm.set_duty(Channel::Ch1, speed.abs() as u16);
            }
            else if lclk == Level::Low && ldt == Level::High {
                speed = speed - 5;
                if speed < -1 * (max_duty as i16) {
                    speed = -1 * (max_duty as i16);
                }
                rprintln!("INFO: PWM duty {}", speed);
                pwm.set_duty(Channel::Ch1, speed.abs() as u16);
            }
            else if lclk == Level::Low && ldt == Level::Low {
                speed = speed + 5;
                if speed > max_duty as i16 {
                    speed = max_duty as i16;
                }
                rprintln!("INFO: PWM duty {}", speed);
                pwm.set_duty(Channel::Ch1, speed.abs() as u16);
            }

            if speed >= 0 {
                an1.set_low();
                an2.set_high();
            }
            else {
                an1.set_high();
                an2.set_low();
            }
        }
    }
}
