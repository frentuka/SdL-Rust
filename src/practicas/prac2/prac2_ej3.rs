/*
Definir la función llamada suma_pares que recibe como parámetro un arreglo de números enteros
y retorna la suma de los números pares.
 */

fn suma_pares(arr: Vec<u32>) -> u32 {
    let mut suma: u32 = 0;
    for num in arr {
        if num % 2 == 0 { suma+= num; }
    }
    suma
}