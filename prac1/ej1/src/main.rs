fn main() {
    println!("Hello, world!asasd");
}

fn leerFloat(panic_msg: &str) -> f32{
    let mut inp: String = String::new();
    stdin().read_line(&mut inp).expect("String input");
    match inp.trim().parse::<f32>() {
        Ok(num) => num,
        _ => panic!("{}", panic_msg)
    }
}