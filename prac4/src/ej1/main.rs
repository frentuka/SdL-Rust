/*
    1- Escriba una función que reciba un vector de números enteros y retorna la cantidad de números primos.
        Cree un trait para la determinación del número primo e impleméntelo según corresponda.
        Utilice la función iter sobre el vector y aplique un closure para resolverlo.
 */

pub trait VerificadorPrimos {
    fn contar_primos(&self) -> usize;
}

impl<T> VerificadorPrimos for Vec<T> where T: Copy + Into<i128>
{
    fn contar_primos(&self) -> usize {
        self.iter().filter(
            |&&n| { // closure
                let n_i64: i128 = n.into();
                verificar_primo(n_i64)
            }
        ).count()
    }
}

// se podria optimizar muchísimo más
fn verificar_primo(num: i128) -> bool {
    if num % 2 == 0 { return false }

    // no me interesa si es divisible por 1
    // sé que no es divisible por 2
    // por eso el primer número es 3
    for n in 3..num.abs() {
        if num % n != 0 { return false }
    }

    true
}

fn main() {
    let i_vec: Vec<i32> = vec![-5, -10, 5, 10];
    let u_vec: Vec<u32> = vec![5, 10, 5, 10];

    println!("iVec: {}", i_vec.contar_primos());
    println!("uVec: {}", u_vec.contar_primos());
}