/*
Definir la función llamada multiplicar_valores que recibe como parámetro un arreglo de enteros
y otro número entero llamado factor.
Esta función multiplica los valores del arreglo por el parámetro factor modificándolo.
 */

fn multiplicar_valores(mut arr: Vec<i32>, factor: i32) {
    arr.iter_mut().for_each(|mut num| *num = *num*factor);
}