/*
    Definir una función llamada incrementar que
    recibe como parámetro un número flotante e incrementa en 1 su valor.
 */

fn main() {

}

fn incrementar(arr: Vec<f32>) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::with_capacity(arr.len());
    for num in arr {
        res.push(num+1.0);
    }
    res
}