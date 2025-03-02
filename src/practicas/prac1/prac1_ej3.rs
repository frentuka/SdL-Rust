use std::io::stdin;

/*
    Escribir un programa que defina una variable de tipo booleano
    y luego permita al usuario ingresar un valor booleano por teclado
    para actualizar su valor haciendo las operaciones and y or.
    Se deben imprimir ambos resultados.
 */

pub fn ej3() {
    let mi_bool = true;

    println!("{mi_bool} AND?");
    let bool_and = leerBool();

    println!("{mi_bool} OR?");
    let bool_or = leerBool();

    println!("{mi_bool} AND {bool_and} = {}", mi_bool && bool_and);
    println!("{mi_bool} OR {bool_or} = {}", mi_bool || bool_or);
}

fn leerBool() -> bool {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("bool input");
    input.trim().parse::<bool>().unwrap()
}