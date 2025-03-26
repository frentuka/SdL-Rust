/*
    Definir la función llamada sumar_arreglos que recibe 2 arreglos del mismo tamaño
        de números flotantes y retorna un nuevo arreglo
        que contiene la suma de los elementos de los arreglos
        pasados por parámetro, correspondiendose el resultado con cada posición
        de los arreglos pasados por parámetro.
 */

fn main() {
    
}

fn sumar_arreglos(arr1: Vec<f32>, arr2: Vec<f32>) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::with_capacity(arr1.len());

    for i in 0..arr1.len() {
        res.push(arr1[i] + arr2[i]);
    }
    
    res
}