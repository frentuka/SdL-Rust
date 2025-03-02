/**
 * Shadowing es diferente a mutabilidad
 *
 * Shadowing es la capacidad de redefinir una variable con el mismo nombre
 * y cambiar su tipo o valor.
 *
 * Mutabilidad es la capacidad de cambiar el valor de una variable sin cambiar su tipo.
 */
pub fn shadowing() {
    let x = 5; // x = 5
    let x = x + 1; // x = 6

    println!("x: {}", x);

    {
        let x = x * 2; // x = 12
        println!("x: {}", x);
    }

    println!("x: {}", x); // x = 6
}

pub fn shadowing2() {
    let texto = "hola mucho texto"; // es un &str
    let texto = texto.len(); // es un entero positivo
    println!("texto: {}", texto); // texto: 16
}