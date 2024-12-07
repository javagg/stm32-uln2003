#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use embassy_stm32::{bind_interrupts, peripherals, timer};

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

// bind_interrupts!(struct Irqs {
//     TIM2 => timer::CaptureCompareInterruptHandler<peripherals::TIM2>;
// });

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello ULN2003 and 28BYJ-48");

    unwrap!(spawner.spawn(blinky(p.PC13)));
    
    let mut coil4 = Output::new(p.PA1, Level::High, Speed::Low);
    let mut coil3 = Output::new(p.PA2, Level::High, Speed::Low);
    let mut coil2 = Output::new(p.PA3, Level::High, Speed::Low);
    let mut coil1 = Output::new(p.PA4, Level::High, Speed::Low);

    let delay = 100;
    loop {
        let steps = 128;
        for _i in 0..steps {
            coil4.set_high();
            coil3.set_low();
            coil2.set_low();
            coil1.set_low();
            Timer::after_millis(delay).await;
            coil4.set_high();
            coil3.set_high();
            coil2.set_low();
            coil1.set_low();
            Timer::after_millis(delay).await;
            coil4.set_low();
            coil3.set_high();
            coil2.set_low();
            coil1.set_low();
            Timer::after_millis(delay).await;
            coil4.set_low();
            coil3.set_high();
            coil2.set_high();
            coil1.set_low();
            Timer::after_millis(delay).await;            
            coil4.set_low();
            coil3.set_low();
            coil2.set_high();
            coil1.set_low();
            Timer::after_millis(delay).await;
            coil4.set_low();
            coil3.set_low();
            coil2.set_high();
            coil1.set_high();
            Timer::after_millis(delay).await;            
            coil4.set_low();
            coil3.set_low();
            coil2.set_low();
            coil1.set_high();
            Timer::after_millis(delay).await;
            coil4.set_high();
            coil3.set_low();
            coil2.set_low();
            coil1.set_high();
            Timer::after_millis(delay).await;            
        }

        for _i in 0..steps {
            coil4.set_high();
            coil3.set_low();
            coil2.set_low();
            coil1.set_high();
            Timer::after_millis(delay).await;
            coil4.set_low();
            coil3.set_low();
            coil2.set_low();
            coil1.set_high();
            Timer::after_millis(delay).await;
            coil4.set_low();
            coil3.set_low();
            coil2.set_high();
            coil1.set_high();
            Timer::after_millis(delay).await;
            coil4.set_low();
            coil3.set_low();
            coil2.set_high();
            coil1.set_low();
            Timer::after_millis(delay).await;
            coil4.set_low();
            coil3.set_high();
            coil2.set_high();
            coil1.set_low();
            Timer::after_millis(delay).await;
            coil4.set_low();
            coil3.set_high();
            coil2.set_low();
            coil1.set_low();
            Timer::after_millis(delay).await;
            coil4.set_high();
            coil3.set_high();
            coil2.set_low();
            coil1.set_low();
            Timer::after_millis(delay).await;
            coil4.set_high();
            coil3.set_low();
            coil2.set_low();
            coil1.set_low();
            Timer::after_millis(delay).await;
        }
    }
}