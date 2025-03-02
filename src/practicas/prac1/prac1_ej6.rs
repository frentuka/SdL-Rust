use std::io::stdin;

/*
    Escribir un programa que defina una variable de tipo entero sin signo
    y luego permita al usuario ingresar un número entero por teclado para sumarse con la variable definida.
    El programa debe imprimir el valor del número elevado al cuadrado.
 */

pub fn ej6() {
    let num: u32 = 13548;
    println!("El número a operar es {}", num);

    let mut input = String::new();

    println!("Ingrese un número (u32) para sumar al operador:");
    stdin().read_line(&mut input).expect("u32 input");
    let num_suma = input.trim().parse::<u32>().unwrap();

    let suma = num + num_suma;
    let cuadrado = suma.pow(2);

    println!("Suma: {}", suma);
    println!("Cuadrado: {}", cuadrado);
}