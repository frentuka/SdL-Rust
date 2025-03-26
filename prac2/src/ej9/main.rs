/*
    Definir la función llamada cantidad_en_rango que recibe 3 parámetros:
        1 arreglo de enteros
        un número entero llamado inferior
        y otro número entero llamado superior.
        Esta función retorna la cantidad de números del arreglo que están
        entre el rango de los parámetros inferior y superior inclusive.
 */

fn main() {

}

fn cantidad_en_rango(arr: Vec<i32>, inf: i32, sup: i32) -> u32 {
    let mut res: u32 = 0;

    for num in arr {
       if num <= sup && num >= inf { res+= 1 }
    }

    res
}