use std::io::stdin;

fn main() {
    let mi_bool = true;

    println!("{mi_bool} AND?");
    let bool_and = leer_bool();

    println!("{mi_bool} OR?");
    let bool_or = leer_bool();

    println!("{mi_bool} AND {bool_and} = {}", mi_bool && bool_and);
    println!("{mi_bool} OR {bool_or} = {}", mi_bool || bool_or);
}

fn leer_bool() -> bool {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("bool input");
    input.trim().parse::<bool>().unwrap()
}