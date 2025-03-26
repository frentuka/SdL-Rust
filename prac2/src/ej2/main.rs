/*
    2- Definir la función llamada es_primo que recibe un número entero positivo mayor a 1
        y retorna true si es primo, false caso contrario.
 */

fn main() {
    for num in 1..100 {
        println!("{num}: {}", es_primo(num));
    }

}

fn es_primo(num: u32) -> bool {
    for i in 2..num  {
        if num % i == 0 {
            return false
        }
    }

    true
}