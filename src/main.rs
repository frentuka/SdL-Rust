use std::fmt::Error;

fn main() {
    let pi = std::f32::consts::PI;
    let e = std::f32::consts::E;
    let result = div(pi, e);

    match result {
        Ok(value) => println!("pi / e = {}", value),
        Err(_) => println!("Error: Division by zero")
    }
}

fn div(a: f32, b: f32) -> Result<f32, Error> {
    if b == 0.0 {
        return Err(Error);
    }

    Ok(a / b)
}