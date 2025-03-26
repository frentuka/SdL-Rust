use std::io::stdin;

fn main() {
    let arr = vec![1, 4, 7, 8, 13548, 13548, 13548, 7];

    let mut inp = String::new();
    println!("Ingrese un número a corroborar de la cadena");
    stdin().read_line(&mut inp).expect("i32 input");
    let inp = inp.trim().parse::<i32>().expect("i32 input");

    let cont = arr.iter().filter(|&x| *x == inp).count();
    println!("En el array {arr:?} hay {cont} elementos iguales a {inp}");
}
