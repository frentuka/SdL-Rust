use std::io::stdin;

fn main() {
    let arr: [&str; 5] = ["hola", "mundo", "como", "estas", "hoy"];
    let mut cadena = String::new();

    println!("Ingrese una cadena:");
    stdin().read_line(&mut cadena).unwrap();

    if arr.contains(&cadena.trim()) {
        println!("La cadena se encuentra en el arreglo.");
    } else {
        println!("La cadena no se encuentra en el arreglo.");
    }
}