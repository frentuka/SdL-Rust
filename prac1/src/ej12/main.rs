fn main() {
    let t: (&str, Vec<u32>) = ("josejuanjo", [1, 2, 3, 4, 5].to_vec());

    let suma: u32 = t.1.iter().sum();

    println!("{} {}", t.0, suma);
}
