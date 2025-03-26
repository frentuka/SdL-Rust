/*
    Definir la función llamada multiplicar_valores que recibe como parámetro
    un arreglo de enteros y otro número entero llamado factor.
    Esta función multiplica los valores del arreglo por el parámetro factor modificándolo.
 */

fn main() {
    const HHHH: &str = "hola";
}

fn multiplicar_valores(arr: Vec<i32>, factor: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::with_capacity(arr.len());

    arr.iter().for_each(|num| res.push(*num*factor));

    res
}