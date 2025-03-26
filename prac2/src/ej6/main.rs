/*
    Definir la función llamada longitud_de_cadenas que recibe un arreglo de String
        y retorna un arreglo con la longitud de las cadenas del parámetro,
            correspondiéndose en posición del arreglo.
 */

fn main() {
    println!("{:?}", longitud_de_cadenas(["hola!".to_string(), "soy".to_string(), "pedro".to_string()].to_vec()))
}

fn longitud_de_cadenas(arr: Vec<String>) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::with_capacity(arr.len());
    for txt in arr {
        res.push(txt.len() as u32);
    }
    res
}