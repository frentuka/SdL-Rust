/*
Definir la función llamada es_par que recibe como parámetro un número entero
y retorna true si el número es par, false caso contrario.
 */
use std::ops::Div;

fn es_par(num: i32) -> bool {
    num.div(2) * 2 == num
}