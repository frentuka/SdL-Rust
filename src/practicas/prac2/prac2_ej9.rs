/*
Definir la función llamada cantidad_en_rango que recibe 3 parámetros:
    un arreglo de enteros, un número entero llamado inferior y otro número entero llamado superior.

Esta función retorna la cantidad de números del arreglo que están entre el rango de los parámetros,
inferior y superior inclusive.
 */

fn cantidad_en_rango(arr: Vec<i32>, lim_sup: i32, lim_inf: i32) -> u32 {
    let mut cant: u32 = 0;
    for num in arr {
        if num >= lim_inf && num <= lim_sup { cant+= 1; }
    }
    cant
}