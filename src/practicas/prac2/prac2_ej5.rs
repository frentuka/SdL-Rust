/*
Defina la función llamada duplicar_valores que recibe un arreglo de números flotantes
y retorna un arreglo nuevo con los valores duplicados del parámetro.
 */

fn duplicar_valores(arr: Vec<f32>) -> Vec<f32> {
    let mut new_arr: Vec<f32> = Vec::with_capacity(arr.len());
    for num in arr {
        new_arr.push(num*2.0);
    }
    new_arr
}