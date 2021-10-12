#![no_main]
#![no_std]

use core::cell::RefCell;
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use hal::{
    adc::{config::AdcConfig, Adc},
    spi::{NoMiso, Spi},
};
use ili9341::{DisplaySize240x320, Ili9341, Orientation, SPI_MODE};
use moist::{sensor::Sensor, ui};
use panic_semihosting as _;

use crate::hal::{prelude::*, stm32};
use stm32f4xx_hal as hal;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let _start = cortex_m_rt::heap_start() as usize;
        let _size = 1024;

        let rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(160.mhz()).freeze();

        let gpioa = p.GPIOA.split();
        let gpioc = p.GPIOC.split();
        let gpiod = p.GPIOD.split();
        let gpiof = p.GPIOF.split();
        let gpiog = p.GPIOG.split();

        let spi = Spi::spi5(
            p.SPI5,
            (
                gpiof.pf7.into_alternate_af5(),
                NoMiso,
                gpiof.pf9.into_alternate_af5(),
            ),
            SPI_MODE,
            20.mhz().into(),
            clocks,
        );

        let cs = gpioc.pc2.into_push_pull_output();
        let dc = gpiod.pd13.into_push_pull_output();

        let if_spi = display_interface_spi::SPIInterface::new(spi, dc, cs);
        let en = gpiof.pf10.into_push_pull_output();
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        let mut lcd = Ili9341::new(
            if_spi,
            en,
            &mut delay,
            Orientation::PortraitFlipped,
            DisplaySize240x320,
        )
        .unwrap();

        let mut plant0 = gpioc.pc0.into_analog();
        let mut plant1 = gpioa.pa5.into_analog();
        let mut plant2 = gpioc.pc1.into_analog();
        let mut plant3 = gpioc.pc3.into_analog();

        // These sensors are all polled one-by-one, so a plain RefCell is sufficient
        // to allow sharing the ADC peripheral across multiple sensors
        let adc = RefCell::new(Adc::adc1(p.ADC1, true, AdcConfig::default()));
        let mut read_plant0 = || adc.borrow_mut().read(&mut plant0).ok();
        let mut read_plant1 = || adc.borrow_mut().read(&mut plant1).ok();
        let mut read_plant2 = || adc.borrow_mut().read(&mut plant2).ok();
        let mut read_plant3 = || adc.borrow_mut().read(&mut plant3).ok();

        let mut world = ui::World::default();

        // Calibration values from dunking in a glass of water and drying off
        // Very scientific...
        world
            .sensors
            .push(Sensor::new(&mut read_plant0, 3530, 1730))
            .ok();
        world
            .sensors
            .push(Sensor::new(&mut read_plant1, 3465, 1730))
            .ok();
        world
            .sensors
            .push(Sensor::new(&mut read_plant2, 3465, 1730))
            .ok();
        world
            .sensors
            .push(Sensor::new(&mut read_plant3, 3600, 1860))
            .ok();

        let user_button = gpioa.pa0.into_pull_down_input();

        let mut green_led = gpiog.pg13.into_push_pull_output();
        green_led.set_high().ok();

        ui::draw_bg(&mut lcd).unwrap();
        loop {
            world.show_raw = user_button.is_high().unwrap_or_default();
            ui::draw_ui(&mut lcd, &mut world).unwrap();
            delay.delay_ms(1000u16 / 30);
        }
    }
    // Should never get here!
    panic!()
}
