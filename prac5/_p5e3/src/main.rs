// 3- En base al ejercicio 9 del tp#3 implemente lo siguiente:
//  a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage de por lo menos 90%
//  b- Ahora el registro de atenciones debe persistir en un archivo en formato JSON,
//      es decir todas la operaciones que lectura, agregar y modificación de atenciones se realizan sobre un archivo.
//
//      No debe modificar los tests hechos en el punto a.
//      Si puede agregar más en caso de que haga métodos nuevos para cumplir con este punto.
//      Recuerde también que se debe seguir manteniendo un coverage de al menos 90%.

mod structs;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::main;

    #[test]
    fn test_main() {
        main()
    }
}