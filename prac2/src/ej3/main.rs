/*
    Definir la función llamada suma_pares que recibe como parámetro un arreglo de números enteros
        y retorna la suma de los números pares.
 */

fn main() {

}

fn suma_pares(arr: Vec<i32>) -> i32 {
    let mut suma: i32 = 0;

    for num in arr {
        if num % 2 == 0 { suma+= num }
    }

    suma
}