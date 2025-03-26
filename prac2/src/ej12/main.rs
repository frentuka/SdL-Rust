/*
    Definir una función llamada reemplazar_pares que recibe un arreglo de enteros
    y reemplaza todos los números pares por -1.
 */

fn main() {

}

fn reemplazar_pares(arr: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::with_capacity(arr.len());
    arr.iter().for_each(|num| res.push(if num % 2 == 0 { -1 } else { *num }));
    res
}