/*
    Escribir un programa que defina una variable de tipo arreglo que contenga seis números enteros
    y luego multiplique cada valor del arreglo por un valor constante definido,
    modificando el contenido del arreglo.
 */

pub fn ej7() {
    let mut arr = vec![1, 2, 3, 4, 5, 6];
    const _MULTIPLICADOR: u16 = 31;

    for num in arr.iter_mut() {
        *num *= _MULTIPLICADOR;
    }

    let arr = arr; // arr is now immutable

    println!("{:?}", arr);
}