use std::io::{stdin, Error};

/*
    Escribir un programa que defina una variable de tipo flotante con algún valor
    y luego permita al usuario ingresar un número decimal por teclado
    para multiplicar, dividir, sumar y restar su valor. Se deben imprimir los resultados.
 */

pub fn ej1() {
    let num: f32 = 13548.0;
    println!("El número a operar es {}", num);
    const FLOAT_ERROR_MSG: &str = "Error convirtiendo input (&str) en f32";

    println!("Ingrese un número (f32) para sumar al operador:");
    let num_suma = leerFloat(FLOAT_ERROR_MSG);

    println!("Ingrese un número (f32) para restar al operador:");
    let num_resta = leerFloat(FLOAT_ERROR_MSG);

    println!("Ingrese un número (f32) para multiplicar al operador:");
    let num_mul = leerFloat(FLOAT_ERROR_MSG);

    println!("Ingrese un número (f32) para dividir al operador:");
    let num_div = leerFloat(FLOAT_ERROR_MSG);

    println!("Suma: {num} + {num_suma} = {}", num + num_suma);
    println!("Resta: {num} - {num_resta} = {}", num - num_resta);
    println!("Multiplicación: {num} * {num_mul} = {}", num * num_mul);
    println!("División: {num} / {num_div} = {}", num / num_div);
}

fn leerFloat(panic_msg: &str) -> f32{
    let mut inp: String = String::new();
    stdin().read_line(&mut inp).expect("String input");
    match inp.trim().parse::<f32>() {
        Ok(num) => num,
        _ => panic!("{}", panic_msg)
    }
}