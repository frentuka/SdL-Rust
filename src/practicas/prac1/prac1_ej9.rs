/*
Escribir un programa que defina un arreglo de 5 números enteros
y luego imprima la suma de los valores del arreglo.
 */

pub fn ej9() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let suma = arr.iter().sum();

    println!("Suma de los valores: {}", suma);
}