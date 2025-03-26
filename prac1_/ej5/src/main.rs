use std::io::stdin;

fn main() {
    let cadena = "asd ";
    // concatenar:

    let mut input = String::new();
    stdin().read_line(&mut input).expect("String input");

    let cadena = cadena.to_string() + &input;

    println!("{}", cadena.to_uppercase());
}
