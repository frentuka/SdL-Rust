fn main() {
    let mut arr = vec![1, 2, 3, 4, 5, 6];
    const _MULTIPLICADOR: u16 = 31;

    for num in arr.iter_mut() {
        *num *= _MULTIPLICADOR;
    }

    let arr = arr; // arr is now immutable

    println!("{:?}", arr);
}
