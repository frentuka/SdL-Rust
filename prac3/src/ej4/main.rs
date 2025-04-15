/*
    4- Escribir un programa que defina la estructura Triángulo que tenga campos para las
    longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos:
        ➢ new: que pasando los parámetros correspondientes, crea un Triángulo y lo retorna.
        ➢ determinar_tipo: retorna el tipo del triángulo, los tipos pueden ser equilátero, isósceles o escaleno.
        ➢ calcular_area: calcular el área y la retorna.
        ➢ calcular_perimetro: calcula el perímetro y lo retorna.
*/

enum TrianguloTypes {
    EQUILATERO, ISOSCELES, ESCALENO
}

struct Triangulo {
    a: f32,
    b: f32,
    c: f32
}

impl Triangulo {
    fn new(a: f32, b: f32, c: f32) -> Triangulo {
        Triangulo {a, b, c}
    }

    fn determinar_tipo(&self) -> TrianguloTypes {
        if self.a == self.b && self.b == self.c {
            return TrianguloTypes::EQUILATERO
        }

        if self.a == self.b
        || self.a == self.c
        || self.b == self.c { return TrianguloTypes::ISOSCELES }
        
        TrianguloTypes::ESCALENO
    }

    fn calcular_area(&self) -> f32 {
        let s = (&self.a + &self.b + &self.c) / 2.0;
        // √[s(s - a)(s - b)(s - c)],
        (s * ((s-&self.a)*(s-&self.b)*(s-&self.c))).sqrt()
    }

    fn calcular_perimetro(&self) -> f32 {
        &self.a + &self.b + &self.c
    }

}

fn main() {
    
}