/*

6- En base al ejercicio 5 del tp#4 implemente lo siguiente:
    a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
            de por lo menos 90%
    b- Todos los balances de los usuarios así como las transacciones
        deben persistir en archivos en formato JSON.

    No debe modificar los tests hechos en el punto a.
    Si puede agregar más en caso de que haga métodos nuevos para cumplir con este punto.
    Recuerde también que se debe seguir manteniendo un coverage de al menos 90%.

 */

#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(unused_must_use)]

mod structs;



fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::main;

    #[test]
    fn test_main() {
        main();
    }
}