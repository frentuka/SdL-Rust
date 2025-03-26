/*
    Definir la función llamada cantidad_de_cadenas_mayor_a que recibe como parámetros
    un arreglo de String y un entero llamado límite.
    Esta función retorna la cantidad de Strings del arreglo que son de longitud mayor
    al parámetro límite
 */

fn main() {

}

fn cantidad_de_cadenas_mayor_a(arr: Vec<String>, limite: u32) -> u32 {
    let mut res: u32 = 0;

    for str in arr {
        if str.len() > limite as usize { res+= 1 }
    }

    res
}