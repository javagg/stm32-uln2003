#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use embassy_stm32::peripherals;

use {defmt_rtt as _, panic_probe as _};

/// Connect PA0 and PC13 with a 1k Ohm resistor

#[embassy_executor::task]
async fn blinky(led: peripherals::PC13) {
    let mut led = Output::new(led, Level::High, Speed::Low);
    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(300).await;
        info!("low");
        led.set_low();
        Timer::after_millis(300).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello ULN2003 and 28BYJ-48");

    unwrap!(spawner.spawn(blinky(p.PC13)));
    
    let mut pin1 = Output::new(p.PA1, Level::High, Speed::Low);
    let mut pin2 = Output::new(p.PA2, Level::High, Speed::Low);
    let mut pin3 = Output::new(p.PA3, Level::High, Speed::Low);
    let mut pin4 = Output::new(p.PA4, Level::High, Speed::Low);

    let delay = 1000;
    loop {
        let mut _steps = 256;
        for _i in 0.._steps {
            pin1.set_high();
            pin2.set_low();
            pin3.set_low();
            pin4.set_low();
            Timer::after_micros(delay).await;
            pin1.set_high();
            pin2.set_high();
            pin3.set_low();
            pin4.set_low();
            Timer::after_micros(delay).await;
            pin1.set_low();
            pin2.set_high();
            pin3.set_low();
            pin4.set_low();
            Timer::after_micros(delay).await;
            pin1.set_low();
            pin2.set_high();
            pin3.set_high();
            pin4.set_low();
            Timer::after_micros(delay).await;            
            pin1.set_low();
            pin2.set_low();
            pin3.set_high();
            pin4.set_low();
            Timer::after_micros(delay).await;
            pin1.set_low();
            pin2.set_low();
            pin3.set_high();
            pin4.set_high();
            Timer::after_micros(delay).await;            
            pin1.set_low();
            pin2.set_low();
            pin3.set_low();
            pin4.set_high();
            Timer::after_micros(delay).await;
            pin1.set_high();
            pin2.set_low();
            pin3.set_low();
            pin4.set_high();
            Timer::after_micros(delay).await;            
        }
        Timer::after_millis(500).await;
        let mut _steps = 128;
        for _i in 0.._steps {
            pin1.set_high();
            pin2.set_low();
            pin3.set_low();
            pin4.set_high();
            Timer::after_micros(delay).await;
            pin1.set_low();
            pin2.set_low();
            pin3.set_low();
            pin4.set_high();
            Timer::after_micros(delay).await;
            pin1.set_low();
            pin2.set_low();
            pin3.set_high();
            pin4.set_high();
            Timer::after_micros(delay).await;
            pin1.set_low();
            pin2.set_low();
            pin3.set_high();
            pin4.set_low();
            Timer::after_micros(delay).await;
            pin1.set_low();
            pin2.set_high();
            pin3.set_high();
            pin4.set_low();
            Timer::after_micros(delay).await;
            pin1.set_low();
            pin2.set_high();
            pin3.set_low();
            pin4.set_low();
            Timer::after_micros(delay).await;
            pin1.set_high();
            pin2.set_high();
            pin3.set_low();
            pin4.set_low();
            Timer::after_micros(delay).await;
            pin1.set_high();
            pin2.set_low();
            pin3.set_low();
            pin4.set_low();
            Timer::after_micros(delay).await;
        }
        Timer::after_millis(500).await;
    }
}