use serde::Serialize;
use std::{
    convert::TryFrom,
    fmt::{self, Display},
};

/// Temperature with a precision of 2
#[derive(Copy, Clone, Debug, Serialize)]
pub struct Celsius(i16);

// can't represent the planck temperature with i16 so absolute zero is enough
#[derive(thiserror::Error, Debug)]
#[error("Received temperature lower than absolute zero: {0}")]
pub struct CelsiusError(i16);

impl TryFrom<i16> for Celsius {
    type Error = CelsiusError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value < -273_15 {
            Err(CelsiusError(value))
        } else {
            Ok(Self(value))
        }
    }
}

impl Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:0>2}°C", self.0 / 100, self.0 % 100)
    }
}

impl Celsius {
    pub fn as_i16(self) -> i16 {
        self.0
    }
}

/// Humidity with a precision of 2 in percent
#[derive(Copy, Clone, Debug, Serialize)]
pub struct RelativeHumidity(u16);

impl RelativeHumidity {
    pub fn as_u16(self) -> u16 {
        self.0
    }
}

impl Display for RelativeHumidity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:0>2}%", self.0 / 100, self.0 % 100)
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Invalid relative humidity, can't be higher than 100%, received {0}")]
pub struct RelativeHumidityError(u16);

impl TryFrom<u16> for RelativeHumidity {
    type Error = RelativeHumidityError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 100_00 {
            Err(RelativeHumidityError(value))
        } else {
            Ok(Self(value))
        }
    }
}

/// Pressure in Pascal with a precision of 1
#[derive(Copy, Clone, Debug, Serialize)]
pub struct Pascal(u32);

impl Pascal {
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl From<u32> for Pascal {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Display for Pascal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:0>1}Pa", self.0 / 10, self.0 % 10)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn relative_humidity_display() {
        assert_eq!(
            RelativeHumidity::try_from(80_01).unwrap().to_string(),
            "80.01%".to_string()
        );

        assert_eq!(
            RelativeHumidity::try_from(100_00).unwrap().to_string(),
            "100.00%".to_string()
        );
    }

    #[test]
    fn relative_humidity_convert() {
        assert!(RelativeHumidity::try_from(100_01).is_err());
        assert!(RelativeHumidity::try_from(140_00).is_err());
        assert!(RelativeHumidity::try_from(10_01).is_ok());
    }

    #[test]
    fn celsius_display() {
        assert_eq!(Celsius::try_from(100_00).unwrap().to_string(), "100.00°C")
    }

    #[test]
    fn celsius_convert() {
        assert!(Celsius::try_from(-320_00).is_err());
        assert!(Celsius::try_from(100_00).is_ok())
    }

    #[test]
    fn pascal_display() {
        assert_eq!(Pascal::from(1000).to_string(), "100.0Pa".to_string())
    }
}
