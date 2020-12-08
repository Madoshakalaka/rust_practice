use rust_practice::units::{self,RA,DEC};

fn calculate_angular_separation() {
    println!("Getting equatorial coordinates for the first star");
    let ra1 = RA::from_stdin();
    let dec1 = DEC::from_stdin();

    println!("Getting equatorial coordinates for the second star");
    let ra2 = RA::from_stdin();
    let dec2 = DEC::from_stdin();

    let ra1_angle = ra1.to_angle();
    // let ra1_sin = ra1_angle.sin();
    // let ra1_cos = ra1_angle.cos();

    let dec1_angle = dec1.to_angle();
    let dec1_sin = dec1_angle.sin();
    let dec1_cos = dec1_angle.cos();


    let ra2_angle = ra2.to_angle();
    // let ra2_sin = ra2_angle.sin();
    // let ra2_cos = ra2_angle.cos();

    let dec2_angle = dec2.to_angle();
    let dec2_sin = dec2_angle.sin();
    let dec2_cos = dec2_angle.cos();

    println!("{} {} {} {}", ra1_angle, dec1_angle, ra2_angle, dec2_angle);

    // the these two are equivalent
    // let angle_cos = ra1_sin * dec1_cos * ra2_sin * dec2_cos + ra1_sin * dec1_sin * ra2_sin * dec2_sin + ra1_cos * ra2_cos;
    let angle_cos = dec1_sin * dec2_sin + dec1_cos * dec2_cos * (ra1_angle - ra2_angle).cos();
    let (a, b, c) = units::radian_to_astro(angle_cos.acos());
    println!("degree: {} arcmin: {} arcsec: {}", a, b, c);
}


fn main() {

    calculate_angular_separation()

}
