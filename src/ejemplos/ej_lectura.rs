use std::io::stdin;

pub fn leer() {
    println!("Ingrese texto a leer:");
    let mut texto_leido = String::new();
    stdin().read_line(&mut texto_leido).expect("Text input");
    println!("Texto leido: {}", texto_leido);
}