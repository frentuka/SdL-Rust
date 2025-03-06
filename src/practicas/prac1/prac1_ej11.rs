/*
Escribir un programa que defina un arreglo de 5 cadenas, y luego permita al usuario ingresar una cadena por teclado.
El programa debe imprimir un mensaje si la cadena ingresada por el usuario se encuentra en el arreglo.
 */

pub fn ej11() {
    let arr: [&str; 5] = ["hola", "mundo", "como", "estas", "hoy"];
    let mut cadena = String::new();

    println!("Ingrese una cadena:");
    std::io::stdin().read_line(&mut cadena).unwrap();

    if arr.contains(&cadena.trim()) {
        println!("La cadena se encuentra en el arreglo.");
    } else {
        println!("La cadena no se encuentra en el arreglo.");
    }
}