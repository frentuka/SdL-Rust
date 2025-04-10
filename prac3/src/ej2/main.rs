
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
    longitud: i32,
    ancho: i32
}

impl Rectangulo {
    fn new(longitud: i32, ancho: i32) -> Rectangulo {
        Rectangulo { longitud, ancho }
    }
    
    fn calcular_area(&self) -> i32 {
        self.longitud * self.ancho
    }
    
    fn calcular_perimetro(&self) -> i32 {
        self.longitud*2 + self.ancho*2
    }
    
    fn es_cuadrado(&self) -> bool {
        self.longitud == self.ancho
    }
    
}

fn main() {
    
}


fn vava(asd: &mut String) {
    
}