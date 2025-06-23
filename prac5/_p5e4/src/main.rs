// 4- En base al ejercicio 10 del tp#3 implemente lo siguiente:
//  a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage de por lo menos 90%
//  b- Tanto los libros con sus copias como la administración de préstamos
//      se realizan sobre archivos en formato JSON.
//
//      Realice las modificaciones pertinentes para poder hacerlo así.
//      No debe modificar los tests hechos en el punto a.
//      Si puede agregar más en caso de que haga métodos nuevos para cumplir con este punto.
//      Recuerde también que se debe seguir manteniendo un coverage de al menos 90%.

#![deny(clippy::pedantic)]

mod structs;

fn main() {}

// just to achieve 100.0% coverage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}