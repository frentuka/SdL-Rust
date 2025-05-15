/*
    4- Escribir un programa que defina la estructura Triángulo que tenga campos para las
    longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos:
        ➢ new: que pasando los parámetros correspondientes, crea un Triángulo y lo retorna.
        ➢ determinar_tipo: retorna el tipo del triángulo, los tipos pueden ser equilátero, isósceles o escaleno.
        ➢ calcular_area: calcular el área y la retorna.
        ➢ calcular_perimetro: calcula el perímetro y lo retorna.
*/

#[derive(Debug)]
#[derive(PartialEq)]
enum TrianguloTypes {
    Equilatero, Isosceles, Escaleno
}

struct Triangulo {
    a: f32,
    b: f32,
    c: f32
}

impl Triangulo {
    fn new(a: f32, b: f32, c: f32) -> Result<Triangulo, String> {
        if a == 0.0
            || b == 0.0
            || c == 0.0
        { return Err("Una longitud medible no puede ser 0".to_string()) }
        
        if (a + b) < c
            || (a + c) < b
            || (b + c) < a
        { return Err("La suma de dos catetos debe ser mayor al tercer cateto".to_string()) }
        
        Ok(Triangulo {a, b, c})
    }

    fn determinar_tipo(&self) -> TrianguloTypes {
        if self.a == self.b && self.b == self.c {
            return TrianguloTypes::Equilatero
        }

        if self.a == self.b
        || self.a == self.c
        || self.b == self.c { return TrianguloTypes::Isosceles }
        
        TrianguloTypes::Escaleno
    }

    fn calcular_area(&self) -> f32 {
        let s = (self.a + self.b + self.c) / 2.0;
        // √[s(s - a)(s - b)(s - c)],
        (s * ((s-self.a)*(s-self.b)*(s-self.c))).sqrt()
    }

    fn calcular_perimetro(&self) -> f32 {
        self.a + self.b + self.c
    }

}

fn main() {
    
}

#[cfg(test)]
mod test_triangulo {
    use crate::{Triangulo, TrianguloTypes};
    
    #[test]
    fn test_triangulo_invalido_1() {
        let triangulo_invalido = Triangulo::new(1.0, 1.0, 1000.0);
        assert!(triangulo_invalido.is_err());
    }
    
    #[test]
    fn test_triangulo_invalido_2() {
        let triangulo_invalido = Triangulo::new(0.0, 1.0, 1.0);
        assert!(triangulo_invalido.is_err());
    }
    
    #[test]
    fn test_triangulo_types() {
        if let Ok(triangulo) = Triangulo::new(1.0, 1.0, 1.0) { assert_eq!(triangulo.determinar_tipo(), TrianguloTypes::Equilatero, "Debería ser equilátero"); }
        if let Ok(triangulo) = Triangulo::new(1.0, 1.0, 2.0) { assert_eq!(triangulo.determinar_tipo(), TrianguloTypes::Isosceles, "Debería ser isósceles"); };
        if let Ok(triangulo) = Triangulo::new(1.0, 2.0, 3.0) { assert_eq!(triangulo.determinar_tipo(), TrianguloTypes::Escaleno, "Debería ser escaleno"); }
    }
}