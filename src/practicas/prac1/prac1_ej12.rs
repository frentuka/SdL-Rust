/*
Escribir un programa que defina una tupla que contenga una cadena y un arreglo de enteros
y luego imprima la cadena y la suma de los valores en el arreglo.
 */

pub fn ej12() {
    let t: (&str, Vec<u32>) = ("josejuanjo", [1, 2, 3, 4, 5]);

    let suma: u32 = t.1.iter().sum();

    println!("{} {}", t.0, suma);
}