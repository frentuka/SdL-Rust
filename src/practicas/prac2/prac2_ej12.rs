/*
Definir una función llamada reemplazar_pares que recibe un arreglo de enteros
y reemplaza todos los números pares por -1.
 */

fn reemplazar_pares(mut arr: Vec<i32>) {
    for num in arr.iter_mut() {
        if *num % 2 == 0 { *num = -1; }
    }
}