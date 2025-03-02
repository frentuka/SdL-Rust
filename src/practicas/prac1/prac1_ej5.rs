use std::io::stdin;

/*
    Escribir un programa que defina una variable de tipo cadena
    y luego permita al usuario ingresar una cadena por teclado para concatenar su valor.
    El programa debe imprimir la cadena en mayúsculas.
 */

pub fn ej5() {
    let cadena = "asd ";
    // concatenar:

    let mut input = String::new();
    stdin().read_line(&mut input).expect("String input");

    let cadena = cadena.to_string() + &input;

    println!("{}", cadena.to_uppercase());
}