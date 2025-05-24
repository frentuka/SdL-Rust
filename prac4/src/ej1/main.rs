/*
    1- Escriba una función que reciba un vector de números enteros y retorna la cantidad de números primos.
        Cree un trait para la determinación del número primo e impleméntelo según corresponda.
        Utilice la función iter sobre el vector y aplique un closure para resolverlo.
 */

pub trait VerificadorPrimos {
    fn contar_primos(&self) -> usize;
}

pub trait VerificarPrimo {
    fn es_primo(&self) -> bool;
}

impl<T> VerificadorPrimos for Vec<T> where T: Copy + Into<i128>
{
    fn contar_primos(&self) -> usize {
        self.iter().filter(
            |&&n| { // closure
                let n_i64: i128 = n.into();
                n_i64.es_primo()
            }
        ).count()
    }
}

impl<T> VerificarPrimo for T where T: Copy + Into<i128> {
    fn es_primo(&self) -> bool {
        let num: i128 = (*self).into();
        if num % 2 == 0 { return false }
        
        // no me interesa si es divisible por 1
        // sé que no es divisible por 2
        // por eso el primer número es 3
        for n in 3..num.abs() {
            if num % n == 0 { return false }
        }

        true
    }
}

fn main() { }

#[cfg(test)]
mod tests {
    use crate::VerificadorPrimos;

    #[test]
    fn test() {
        let i_vec: Vec<i32> = vec![-5, -10, 5, 10];
        let u_vec: Vec<u32> = vec![5, 10, 5, 10];
        let p_vec: Vec<u32> = vec![0, 2, 4, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30];

        assert_eq!(i_vec.contar_primos(), 2, "Deberían haber 2 primos en el vector");
        assert_eq!(u_vec.contar_primos(), 2, "Deberian haber 2 primos en el vector");
        assert_eq!(p_vec.contar_primos(), 0, "No deberían haber primos en el vector");
    }

}