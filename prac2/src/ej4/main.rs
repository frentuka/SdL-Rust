/*
    Definir la función llamada cantidad_impares que recibe como parámetro
        un arreglo de números enteros y retorna la cantidad de números impares.
 */

fn main() {

}

fn cantidad_pares(arr: Vec<i32>) -> u32 {
    let mut cant: u32 = 0;
    for num in arr {
        if num % 2 == 0 { cant+= 1 }
    }
    cant
}

#[cfg(test)]
mod tests {
    use crate::cantidad_pares;

    #[test]
    fn test() {
        assert_eq!(cantidad_pares(Vec::new()), 0, "Se esperan 0 números");
    }
    
}