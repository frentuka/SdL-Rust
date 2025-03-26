/*
    Definir la función llamada cantidad_impares que recibe como parámetro
        un arreglo de números enteros y retorna la cantidad de números impares.
 */

fn main() {

}

fn cantidad_pares(arr: Vec<i32>) -> u32 {
    let mut cant: u32 = 0;
    for num in arr {
        if num % 2 == 0 { cant+= 1 }
    }
    cant
}