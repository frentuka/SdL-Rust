
/*
    2- Escribir un programa que defina la estructura Rectángulo que tenga campos para la
        longitud y el ancho. Para dicha estructura implemente los siguientes métodos:
        ➢ new: que pasando los parámetros correspondientes, crea un Rectángulo y lo
        retorna.
        ➢ calcular_area: calcular el área y la retorna.
        ➢ calcular_perimetro: calcula el perímetro y lo retorna.
        ➢ es_cuadrado: retorna true si es cuadrado, false caso contrario
 */

struct Rectangulo {
    longitud: u16,
    ancho: u16
}

impl Rectangulo {
    fn new(longitud: u16, ancho: u16) -> Rectangulo {
        if longitud == 0 || ancho == 0 { panic!("Una longitud medible no puede ser 0") }
        Rectangulo { longitud, ancho }
    }
    
    fn calcular_area(&self) -> u32 {
        self.longitud as u32 * self.ancho as u32
    }
    
    fn calcular_perimetro(&self) -> u32 {
        self.longitud as u32*2 + self.ancho as u32*2
    }
    
    fn es_cuadrado(&self) -> bool {
        self.longitud == self.ancho
    }
}

fn main() { }

#[cfg(test)]
mod tests_rectangulo {
    use crate::Rectangulo;

    #[test]
    fn test_limites_i32() {
        let mi_rectangulo = Rectangulo::new(u16::MAX, u16::MAX);
        let mi_rectangulo_neg = Rectangulo::new(u16::MIN, u16::MIN);
        let mi_rectangulo_0 = Rectangulo::new(0, 0);

        // shouldn't panic
        let area = mi_rectangulo.calcular_area();
        let perimetro = mi_rectangulo.calcular_perimetro();
        let cuadrado = mi_rectangulo.es_cuadrado();

        let area = mi_rectangulo_neg.calcular_area();
        let perimetro = mi_rectangulo_neg.calcular_perimetro();
        let cuadrado = mi_rectangulo_neg.es_cuadrado();

        let area = mi_rectangulo_0.calcular_area();
        let perimetro = mi_rectangulo_0.calcular_perimetro();
        let cuadrado = mi_rectangulo_0.es_cuadrado();
    }
}