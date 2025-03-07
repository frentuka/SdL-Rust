/*
Definir la función llamada cantidad_de_mayores que recibe como parámetro un arreglo de números enteros
y un número entero llamado límite.

Esta función retorna la cantidad de números mayores al límite que tiene el arreglo.
 */

fn cantidad_de_mayores(arr: Vec<i32>, limite: i32) -> u32 {
    let mut cant: u32 = 0;
    for num in arr {
        if num > limite { cant+= 1; }
    }
    cant
}