use std::io::stdin;

fn main() {
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
