
/*
    7- Defina una estructura llamada ConcesionarioAuto donde se conoce
            nombre,
            dirección
            y tiene una capacidad máxima para albergar X cantidad de autos.
        De los autos se conocen los campos de
            marca,
            modelo,
            año,
            precio bruto
            y color que pueden ser:
                rojo,
                verde,
                azul,
                amarillo,
                blanco,
                negro.
        Para dichas estructuras implemente los siguientes métodos:
        ❖ ConcesionarioAuto:
            ➢ new: que pasando los parámetros correspondientes, crea un ConcesionarioAuto y lo retorna.
            ➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene
                sin superar la máxima cantidad para albergarlos
                y retorna true, en caso de que lo supere no lo agrega y retorna false.
            ➢ eliminar_auto(auto): elimina un auto de la lista de autos.
            ➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
        ❖ Auto:
            ➢ new: que pasando los parámetros correspondientes, crea un Auto y lo retorna.
            ➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
                ■ si es de color primario le aplica un recargo del 25%, sino le aplica un descuento del 10%.
                ■ si la marca es BMW le aplica un recargo del 15%-
                ■ si el año es menor a 2000 le aplica un descuento del 5%.
 */

struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    autos: Vec<Auto>
}

#[derive(PartialEq)]
struct Auto {
    marca: String,
    modelo: String,
    ano: u16,
    precio: f32,
    color: Color
}

#[derive(PartialEq)]
enum Color {
    Rojo, Verde, Azul, Amarillo, Blanco, Negro
}

impl ConcesionarioAuto {
    // ➢ new: que pasando los parámetros correspondientes, crea un ConcesionarioAuto y lo retorna.
    fn new(nombre: String, direccion: String, autos: Vec<Auto>) -> ConcesionarioAuto {
        ConcesionarioAuto { nombre, direccion, autos }
    }

    // ➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene
    //     sin superar la máxima cantidad para albergarlos
    //     y retorna true, en caso de que lo supere no lo agrega y retorna false.
    fn agregar_auto(&mut self, auto: Auto) -> bool {
        if self.autos.len() == self.autos.capacity() { return false }
        self.autos.push(auto);
        true
    }

    // ➢ eliminar_auto(auto): elimina un auto de la lista de autos.
    fn eliminar_auto(&mut self, auto: Auto) {
        self.autos.retain(|a| *a != auto);
    }

    // ➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
    fn buscar_auto(&self, auto: Auto) -> Option<Auto> {
        if self.autos.contains(&auto) {
            return Some(auto)
        }
        None
    }
}

impl Auto {
    // ➢ new: que pasando los parámetros correspondientes, crea un Auto y lo retorna.
    fn new(marca: String, modelo: String, ano: u16, precio: f32, color: Color) -> Auto {
        Auto { marca, modelo, ano, precio, color }
    }

    // ➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
    fn calcular_precio(&self) -> f32 {
        let mut precio = self.precio;

        // ■ si es de color primario le aplica un recargo del 25%, sino le aplica un descuento del 10%.
        match self.color {
            Color::Rojo | Color::Azul | Color::Amarillo => precio*= 1.25,
            _ => precio/= 0.9
        }

        // ■ si la marca es BMW le aplica un recargo del 15%
        if self.marca == "BMW" { precio*= 1.15 }

        // ■ si el año es menor a 2000 le aplica un descuento del 5%.
        if self.ano < 2000 { precio*= 0.95 }

        precio
    }
}



fn main() {

}