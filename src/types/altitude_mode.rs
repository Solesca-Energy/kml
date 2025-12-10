use std::fmt;
use std::str::FromStr;

use crate::errors::Error;

/// `kml:altitudeMode`, [9.20](http://docs.opengeospatial.org/is/12-007r2/12-007r2.html#322) in the
/// KML specification
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum AltitudeMode {
    #[default]
    ClampToGround,
    RelativeToGround,
    Absolute,
}

impl FromStr for AltitudeMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // While "clampToGround" is the proper terminology
            // per the KML specification, it appears as though
            // Civil3D exports KMZ files with "clampedToGround."
            "clampToGround" | "clampedToGround" => Ok(Self::ClampToGround),
            "relativeToGround" => Ok(Self::RelativeToGround),
            "absolute" => Ok(Self::Absolute),
            v => Err(Error::InvalidAltitudeMode(v.to_string())),
        }
    }
}

impl fmt::Display for AltitudeMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ClampToGround => "clampToGround",
                Self::RelativeToGround => "relativeToGround",
                Self::Absolute => "absolute",
            }
        )
    }
}
