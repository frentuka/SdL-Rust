fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let suma: i32 = arr.iter().sum();

    println!("Suma de los valores: {}", suma);
}
