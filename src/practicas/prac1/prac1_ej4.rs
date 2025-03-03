/*
    Escribir un programa que defina una tupla
    que contenga una cadena, un número entero con signo y un valor booleano
    y luego imprima cada valor de la tupla
 */

pub fn ej4() {
    let mi_tupla = ("ohmama", 13658, false);
    println!("Impresión directa: {mi_tupla:?}");
    println!("Impresión individual: {}, {}, {}", mi_tupla.0, mi_tupla.1, mi_tupla.2);
}