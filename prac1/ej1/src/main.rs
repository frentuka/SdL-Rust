use std::io::stdin;

fn main() {
    let num: f32 = 13548.0;
    println!("El número a operar es {}", num);
    const FLOAT_ERROR_MSG: &str = "Error convirtiendo input (&str) en f32";

    println!("Ingrese un número (f32) para sumar al operador:");
    let num_suma = leer_float(FLOAT_ERROR_MSG);

    println!("Ingrese un número (f32) para restar al operador:");
    let num_resta = leer_float(FLOAT_ERROR_MSG);

    println!("Ingrese un número (f32) para multiplicar al operador:");
    let num_mul = leer_float(FLOAT_ERROR_MSG);

    println!("Ingrese un número (f32) para dividir al operador:");
    let num_div = leer_float(FLOAT_ERROR_MSG);

    println!("Suma: {num} + {num_suma} = {}", num + num_suma);
    println!("Resta: {num} - {num_resta} = {}", num - num_resta);
    println!("Multiplicación: {num} * {num_mul} = {}", num * num_mul);
    println!("División: {num} / {num_div} = {}", num / num_div);
}

fn leer_float(panic_msg: &str) -> f32{
    let mut inp: String = String::new();
    stdin().read_line(&mut inp).expect("String input");
    match inp.trim().parse::<f32>() {
        Ok(num) => num,
        _ => panic!("{}", panic_msg)
    }
}