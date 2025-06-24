#![no_std]
#![no_main]

// mod rtt_logger;

// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

// use cortex_m::asm;
// use cortex_m_rt::entry;
// use embedded_hal::digital::{InputPin, OutputPin};
// use log::{error, info, warn};
// use panic_rtt_target as _;
// use stm32f1xx_hal::{pac, prelude::*, time::Hz, timer::Timer};

// use nb::block;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_time::Timer;
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embassy_stm32::peripherals::I2C1;

use {defmt_rtt as _, panic_probe as _};
use defmt::info;
use defmt::warn;

// #[entry]
// fn main() -> ! {
//     rtt_logger::init(log::LevelFilter::Debug);

//     let cp = cortex_m::Peripherals::take().unwrap();
//     let dp = pac::Peripherals::take().unwrap();

//     let mut flash = dp.FLASH.constrain();

//     let rcc = dp.RCC.constrain();

//     let clocks = rcc.cfgr.freeze(&mut flash.acr);

//     // let mut gpioc = dp.GPIOC.split();
//     // let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

//     let mut gpiob = dp.GPIOB.split();
//     let mut led0 = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
//     let mut led1 = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);

//     let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
//     timer.start(Hz(1)).unwrap();

//     let mut cnt = 0;

//     loop {
//         if cnt == 0 {
//             error!("error info : {}", cnt);
//         } else if cnt < 10 {
//             warn!("warning info : {}", cnt);
//         } else if cnt < 20 {
//             info!("info info : {}", cnt);
//         } else {
//             panic!("warning info : {}", cnt);
//         }
//         cnt += 1;
//         block!(timer.wait()).unwrap();
//         led0.set_high();
//         led1.set_low();
//         block!(timer.wait()).unwrap();
//         led0.set_low();
//         led1.set_high();ca
//     }
// }

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>; 
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // rtt_logger::init(log::LevelFilter::Debug);

    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    

    let mut led = Output::new(p.PB0, Level::High, Speed::Low);


    let mut iic = I2c::new(
        p.I2C1,
        p.PB6,
        p.PB7,
        Irqs,
        NoDma,
        NoDma,
        Hertz(100000),
        i2c::Config::default(),
    );



        // 启动第一个任务
    spawner.spawn(blink_led(led)).unwrap();

    // 启动第二个任务
    spawner.spawn(another_task()).unwrap();
}

#[embassy_executor::task]
async fn blink_led(mut led: Output<'static,embassy_stm32::peripherals::PB0>) {
    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(1000).await;

        info!("low");
        led.set_low();
        Timer::after_millis(1000).await;
    }
}

#[embassy_executor::task]
async fn another_task() {
    loop {
        warn!("Running another task...");
        Timer::after_millis(1000).await;
    }
}