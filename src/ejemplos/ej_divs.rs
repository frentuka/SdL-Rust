use std::fmt::Error;

pub fn divisionExample() {

    let pi32 = std::f32::consts::PI;
    let e32 = std::f32::consts::E;
    let pi64 = std::f64::consts::PI;
    let e64 = std::f64::consts::E;

    let result32 = div_f32(pi32, e32);
    let result64 = div_f64(pi64, e64);

    match result32 {
        Ok(value) => println!("(f32) pi / e = {}", value),
        Err(_) => println!("Error: Division by zero")
    }

    match result64 {
        Ok(value) => println!("(f64) pi / e = {}", value),
        Err(_) => println!("Error: Division by zero")
    }
}

pub fn div_f32(a: f32, b: f32) -> Result<f32, Error> {
    if b == 0.0 {
        return Err(Error);
    }

    Ok(a / b)
}

pub fn div_f64(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        return Err(Error);
    }

    Ok(a / b)
}