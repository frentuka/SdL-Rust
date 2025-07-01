// 5- En base al ejercicio 3 del tp#4 implemente lo siguiente:
//  a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
//      de por lo menos 90%
//  b- Todas las suscripciones deben almacenarse en un archivo en formato JSON,
//      implemente lo necesario para que toda la funcionalidad de las suscripciones se realice
//      guardando, leyendo o modificando archivos.
//
// No debe modificar los tests hechos en el punto a.
// Si puede agregar más en caso de que haga métodos nuevos para cumplir con este punto.
// Recuerde también que se debe seguir manteniendo un coverage de al menos 90%.

// interpretación personal del ejercicio:
// la estructura debe poder persistir completamente en archivos.

#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(unused_must_use)]

mod structs;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}