use core::fmt::Write;
use heapless::String;

// Tuned for the size of the display and the font chosen
pub const MAX_STR_LEN: usize = 13;

pub struct Sensor<'a> {
    read: &'a mut dyn FnMut() -> Option<u16>,
    dry: u16,
    wet: u16,
}

impl<'a> Sensor<'a> {
    pub fn new(read: &'a mut dyn FnMut() -> Option<u16>, dry: u16, wet: u16) -> Self {
        Sensor { read, dry, wet }
    }

    pub fn raw_reading(&mut self) -> RawReading {
        RawReading {
            value: (self.read)(),
            dry: self.dry,
            wet: self.wet,
        }
    }
}

pub struct RawReading {
    value: Option<u16>,
    dry: u16,
    wet: u16,
}

impl From<RawReading> for String<{ MAX_STR_LEN }> {
    fn from(reading: RawReading) -> Self {
        if let Some(reading) = reading.value {
            let mut s = Self::new();
            if write!(&mut s, "{}", reading).is_err() {
                Self::from("Error")
            } else {
                s
            }
        } else {
            Self::from("No Sensor")
        }
    }
}

#[derive(Debug)]
enum ReadingError {
    NoSensor,
    Calibration,
    OutOfRangeHigh,
    OutOfRangeLow,
}

pub struct Percent {
    value: Result<f32, ReadingError>,
}

impl Percent {
    fn calibrated_value(value: u16, at_0: u16, at_100: u16) -> Result<f32, ReadingError> {
        if at_100 == at_0 {
            Err(ReadingError::Calibration)
        } else {
            let value = value as f32;
            let at_0 = at_0 as f32;
            let at_100 = at_100 as f32;
            let result = 100f32 * (value - at_0) / (at_100 - at_0);
            // Allow +/- 20% over-under reporting to account for the real world
            if result > 120.0 {
                Err(ReadingError::OutOfRangeHigh)
            } else if result < -20.0 {
                Err(ReadingError::OutOfRangeLow)
            } else {
                Ok(result)
            }
        }
    }
}

impl From<RawReading> for Percent {
    fn from(raw: RawReading) -> Self {
        Self {
            value: raw.value.map_or(Err(ReadingError::NoSensor), |value| {
                Self::calibrated_value(value, raw.dry, raw.wet)
            }),
        }
    }
}

impl From<Percent> for String<{ MAX_STR_LEN }> {
    fn from(percent: Percent) -> Self {
        match percent.value {
            Ok(percent) => {
                let mut s = Self::new();
                if write!(&mut s, "{:.1}%", percent).is_err() {
                    Self::from("Error")
                } else {
                    s
                }
            }
            Err(ReadingError::Calibration) => Self::from("Uncalibrated"),
            Err(ReadingError::NoSensor) => Self::from("No Sensor"),
            Err(ReadingError::OutOfRangeHigh | ReadingError::OutOfRangeLow) => {
                // Assume unplugged, it could also be a bad calibration
                Self::from("Unplugged")
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // Some trivial smoke tests for converting the ADC reading to a calibrated Percent
    #[test]
    fn test_trivial_calibrated_value() {
        assert_eq!(Percent::calibrated_value(0, 0, 100).ok(), Some(0f32));
        assert_eq!(Percent::calibrated_value(25, 0, 100).ok(), Some(25f32));
        assert_eq!(Percent::calibrated_value(50, 0, 100).ok(), Some(50f32));
        assert_eq!(Percent::calibrated_value(75, 0, 100).ok(), Some(75f32));
        assert_eq!(Percent::calibrated_value(100, 0, 100).ok(), Some(100f32));
    }

    #[test]
    fn test_inverted_trivial_calibrated_value() {
        assert_eq!(Percent::calibrated_value(0, 100, 0).ok(), Some(100f32));
        assert_eq!(Percent::calibrated_value(25, 100, 0).ok(), Some(75f32));
        assert_eq!(Percent::calibrated_value(50, 100, 0).ok(), Some(50f32));
        assert_eq!(Percent::calibrated_value(75, 100, 0).ok(), Some(25f32));
        assert_eq!(Percent::calibrated_value(100, 100, 0).ok(), Some(0f32));
    }
}
