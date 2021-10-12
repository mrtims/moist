use core::time;
use std::{process::exit, thread};

use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use moist::{sensor::Sensor, ui};

fn main() -> () {
    println!("Hello world");

    let mut lcd = SimulatorDisplay::<Rgb565>::new(Size::new(240, 320));

    let mut read_plant0 = || Some(3000);
    let mut read_plant1 = || Some(3000);
    let mut read_plant2 = || Some(3000);
    let mut read_plant3 = || None;

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

    ui::draw_bg(&mut lcd).unwrap();

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Hello World", &output_settings);

    loop {
        ui::draw_ui(&mut lcd, &mut world).unwrap();
        window.update(&lcd);
        window.events().for_each(|event| match event {
            SimulatorEvent::KeyUp {
                keycode,
                keymod: _,
                repeat: _,
            } => {
                if keycode == Keycode::Return {
                    world.show_raw = false;
                }
            }
            SimulatorEvent::KeyDown {
                keycode,
                keymod: _,
                repeat: _,
            } => {
                if keycode == Keycode::Return {
                    world.show_raw = true;
                }
            }
            SimulatorEvent::Quit => exit(0),
            _ => (),
        });

        thread::sleep(time::Duration::from_millis(1000 / 30));
    }
}
