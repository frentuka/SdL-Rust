/*
Definir la función llamada longitud_de_cadenas que recibe un arreglo de String
y retorna un arreglo con la longitud de las cadenas del parámetro, correspondiéndose en posición del arreglo.
 */

fn longitud_de_cadenas(arr: Vec<String>) -> Vec<u32> {
    let mut longitudes: Vec<u32> = Vec::with_capacity(arr.len());
    for str in arr {
        longitudes.push(str.len() as u32);
    }
    longitudes
}