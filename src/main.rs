use std::io::prelude::*;
use std::io;
use std::str::SplitWhitespace;
use std::iter::Filter;
use std::f64;

#[derive(Default)]
struct SpaceSepValGetter {
    input_buffer: String
}

impl SpaceSepValGetter {
    /// this fn gets space separated values from stdin
    fn get_ssv(&mut self, prompt: &str) -> SplitWhitespace {
        self.input_buffer.clear();
        print!("{}", prompt);
        io::stdout().flush().ok().expect("Could not flush stdout");

        io::stdin().read_line(&mut self.input_buffer).expect("Wrong!");
        let args = self.input_buffer.split_whitespace();
        args
    }
}






/// right ascension
#[derive(Debug)]
struct RA {
    hour: u8,
    minute: u8,
    second: f64,
}

impl RA {

    fn from_stdin() -> Self {
        let mut ssv_getter: SpaceSepValGetter = Default::default();
        let mut ssv = ssv_getter.get_ssv("Enter the right ascension (RA) in the form of h m s, separated by spaces: ");



        Self {
            hour: ssv.next().unwrap().parse().unwrap(),
            minute: ssv.next().unwrap().parse().unwrap(),
            second: ssv.next().unwrap().parse().unwrap(),
        }
    }

    // /// return a radian
    // fn angle_difference(ra1:Self, ra2:Self)->f64{
    //     let diff = ((ra1.hour - ra2.hour)*15) as f64 + (ra1.minute - ra2.minute) as f64 / 4f64 + (ra1.second - ra2.second) / 15f64;
    // }

    fn to_angle(&self)->f64{
        std::f64::consts::PI * ((self.hour as f64 * 15f64)  + self.minute as f64 / 4f64 + self.second / 240f64) / 180f64
    }
}

/// Declination
#[derive(Debug)]
struct DEC {
    degree: i16,
    arcmin: i8,
    arcsec: f64,
}

impl DEC {
    fn from_stdin() -> DEC {
        let mut ssv_getter: SpaceSepValGetter = Default::default();
        let mut ssv = ssv_getter.get_ssv("Enter the Declination (DEC) in the form of degree arcmin arcsec, separated by spaces: ");

        let deg = ssv.next().unwrap();
        let degree: i16;
        let mut multiplier:i8 = 1;
        if deg.starts_with('-'){
            degree = deg[1..].parse().unwrap();
            multiplier = -1;
        }else{
            degree = deg.parse().unwrap();
        }

        DEC {
            degree: multiplier as i16 * degree,
            arcmin: multiplier * ssv.next().unwrap().parse::<i8>().unwrap(),
            arcsec: multiplier as f64 * ssv.next().unwrap().parse::<f64>().unwrap(),
        }
    }

    fn to_angle(&self)->f64{
        std::f64::consts::PI * (self.degree as f64 + self.arcmin as f64 / 60f64 + self.arcsec / 3600f64) / 180f64
    }
}


struct EquatorialCoordinate{
    ra: RA,
    dec: DEC
}

// impl EquatorialCoordinate{
//     fn angle_between(c1:EquatorialCoordinate, c2:EquatorialCoordinate){
//
//     }
// }


fn radian_to_astro(radian:f64)->(u16, u8, f64){
    let mut degrees = 180f64 * radian / std::f64::consts::PI;

    let degree_part = degrees as u16;
    degrees = (degrees - degree_part as f64)*60f64;
    let arcmin_part = degrees as u8;
    degrees = (degrees - arcmin_part as f64)*60f64;

    (degree_part, arcmin_part, degrees)
}

fn main() {
    println!("Getting equatorial coordinates for the first star");
    let ra1 = RA::from_stdin();
    let dec1 = DEC::from_stdin();

    println!("Getting equatorial coordinates for the second star");
    let ra2 = RA::from_stdin();
    let dec2 = DEC::from_stdin();

    let ra1_angle = ra1.to_angle();
    let ra1_sin = ra1_angle.sin();
    let ra1_cos = ra1_angle.cos();

    let dec1_angle = dec1.to_angle();
    let dec1_sin = dec1_angle.sin();
    let dec1_cos = dec1_angle.cos();


    let ra2_angle = ra2.to_angle();
    let ra2_sin = ra2_angle.sin();
    let ra2_cos = ra2_angle.cos();

    let dec2_angle = dec2.to_angle();
    let dec2_sin = dec2_angle.sin();
    let dec2_cos = dec2_angle.cos();

    println!("{} {} {} {}", ra1_angle, dec1_angle, ra2_angle, dec2_angle);

    let angle_cos = ra1_sin * dec1_cos * ra2_sin * dec2_cos + ra1_sin * dec1_sin * ra2_sin * dec2_sin + ra1_cos * ra2_cos;
    let angle_cos = dec1_sin * dec2_sin + dec1_cos * dec2_cos * (ra1_angle - ra2_angle).cos();

    let (a, b, c) = radian_to_astro(angle_cos.acos());
    println!("degree: {} arcmin: {} arcsec: {}", a, b, c);

}
