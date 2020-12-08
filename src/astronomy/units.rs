use std::f64::consts::PI;
use core::default::Default;
use crate::io_utils::input_parser::SpaceSepValGetter;

/// right ascension
#[derive(Debug)]
pub struct RA {
    hour: u8,
    minute: u8,
    second: f64,
}

impl RA {
    pub fn from_stdin() -> Self {
        let mut ssv_getter: SpaceSepValGetter = Default::default();
        let mut ssv = ssv_getter.get_ssv("Enter the right ascension (RA) in the form of h m s, separated by spaces: ");
        Self {
            hour: ssv.next().unwrap().parse().unwrap(),
            minute: ssv.next().unwrap().parse().unwrap(),
            second: ssv.next().unwrap().parse().unwrap(),
        }
    }


    pub fn to_angle(&self) -> f64 {
        std::f64::consts::PI * ((self.hour as f64 * 15f64) + self.minute as f64 / 4f64 + self.second / 240f64) / 180f64
    }
}

/// Declination
#[derive(Debug)]
pub struct DEC {
    degree: i16,
    arcmin: i8,
    arcsec: f64,
}

impl DEC {
    pub fn from_stdin() -> DEC {
        let mut ssv_getter: SpaceSepValGetter = Default::default();
        let mut ssv = ssv_getter.get_ssv("Enter the Declination (DEC) in the form of degree arcmin arcsec, separated by spaces: ");

        let deg = ssv.next().unwrap();
        let degree: i16;
        let mut multiplier: i8 = 1;
        if deg.starts_with('-') {
            degree = deg[1..].parse().unwrap();
            multiplier = -1;
        } else {
            degree = deg.parse().unwrap();
        }

        DEC {
            degree: multiplier as i16 * degree,
            arcmin: multiplier * ssv.next().unwrap().parse::<i8>().unwrap(),
            arcsec: multiplier as f64 * ssv.next().unwrap().parse::<f64>().unwrap(),
        }
    }

    pub fn to_angle(&self) -> f64 {
        PI * (self.degree as f64 + self.arcmin as f64 / 60f64 + self.arcsec / 3600f64) / 180f64
    }
}


// struct EquatorialCoordinate {
//     ra: RA,
//     dec: DEC,
// }


pub fn radian_to_astro(radian: f64) -> (u16, u8, f64) {
    let mut degrees = 180f64 * radian / PI;

    let degree_part = degrees as u16;
    degrees = (degrees - degree_part as f64) * 60f64;
    let arcmin_part = degrees as u8;
    degrees = (degrees - arcmin_part as f64) * 60f64;

    (degree_part, arcmin_part, degrees)
}
