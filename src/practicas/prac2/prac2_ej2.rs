/*
Definir la función llamada es_primo que recibe un número entero positivo mayor a 1
y retorna true si es primo, false caso contrario
 */
use std::ops::Div;

pub fn ej2() {
    for i in 1..100 {
        if es_primo(i) { println!("{} es primo", i); }
    }
}

fn es_primo(num: u32) -> bool {
    for i in 2..num {
        if num % i == 0 {
            return false
        }
    }

    true
}