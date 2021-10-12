use crate::sensor::{Percent, Sensor, MAX_STR_LEN};
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::MonoTextStyleBuilder,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
    text::Text,
};
use heapless::{String, Vec};
use profont::PROFONT_24_POINT;

const BORDER_COLOUR: Rgb565 = Rgb565::new(21, 61, 22);
const BACKGROUND_COLOUR: Rgb565 = Rgb565::new(31, 63, 28);
const TEXT_COLOUR: Rgb565 = Rgb565::new(0, 11, 2);

#[derive(Default)]
pub struct World<'a> {
    pub sensors: Vec<Sensor<'a>, 4>,
    pub show_raw: bool,
}

// The LCD Driver has no framebuffer, so padding strings to the end means they look nice even
// after shrinking. It does mean that the MAX_STR_LEN must be tuned to never be larger the size of the display,
// smaller is fine as long as it's longer than the longest possible text.
fn pad_to_end<const N: usize>(str: &mut String<N>) -> &mut String<N> {
    while str.push(' ').is_ok() {}
    str
}

// Split so the background is drawn only once on start-up.
// Drawing everything from scratch each time without a framebuffer introduces visible flickering
// as the background is drawn over the ui.
pub fn draw_bg<D>(target: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    let bg_style = PrimitiveStyleBuilder::new()
        .stroke_color(BORDER_COLOUR)
        .stroke_width(20)
        .fill_color(BACKGROUND_COLOUR)
        .build();
    Rectangle::new(Point::new(0, 0), target.bounding_box().size)
        .into_styled(bg_style)
        .draw(target)?;
    Ok(())
}

pub fn draw_ui<D>(target: &mut D, world: &mut World) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb565>,
{
    let text_style = MonoTextStyleBuilder::<Rgb565>::new()
        .font(&PROFONT_24_POINT)
        .background_color(BACKGROUND_COLOUR)
        .text_color(TEXT_COLOUR)
        .build();

    let title_text = Text::new("Soil Meter:", Point::new(20, 50), text_style);
    title_text.draw(target)?;
    let mut position = title_text.position;

    for sensor in &mut world.sensors {
        position += Point::new(0, 60);
        let reading = sensor.raw_reading();
        let mut as_str: String<{ MAX_STR_LEN }> = if world.show_raw {
            reading.into()
        } else {
            let percent: Percent = reading.into();
            percent.into()
        };

        Text::new(pad_to_end(&mut as_str), position, text_style).draw(target)?;
    }

    Ok(())
}
