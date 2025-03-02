use std::io::stdin;

/*
    Escribir un programa que defina una constante de tipo cadena
    y luego imprima el número de veces que un caracter específico ingresado por el usuario aparece en la cadena.
    Se debe imprimir el resultado.
 */

pub fn ej8() {
    let arr = vec![1, 4, 7, 8, 13548, 13548, 13548, 7];

    let mut inp = String::new();
    println!("Ingrese un número a corroborar de la cadena");
    stdin().read_line(&mut inp).expect("i32 input");
    let inp = inp.trim().parse::<i32>().expect("i32 input");

    let cont = arr.iter().filter(|&x| *x == inp).count();
    println!("En el array {arr:?} hay {cont} elementos iguales a {inp}");
}