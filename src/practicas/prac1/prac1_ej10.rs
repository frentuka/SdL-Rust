/*
Escribir un programa que defina dos arreglos de 5 números enteros cada uno
y luego cree un tercer arreglo que contenga la suma de los elementos de los dos arreglos originales.
 */

pub fn ej10() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [2, 3, 4, 5, 6];
    let mut arr3: Vec<i32> = Vec::with_capacity(5);

    for i in 0..5 {
        arr3[i] = arr1[i] + arr2[i];
    }
}