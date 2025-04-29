/*
    5- Escribir un programa que defina una estructura Producto que tenga campos para el
        nombre, el precio bruto y un número identificatorio. Para dicha estructura implemente los
        siguientes métodos:
        ➢ new: que pasando los parámetros correspondientes, crea un Producto y lo retorna.
        ➢ calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre
        el precio bruto
        ➢ aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de
        descuento sobre el precio bruto
        ➢ calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el
        precio total a pagar aplicando impuesto y descuento. Tenga en cuenta que los
        parámetros son opcionales.
*/

struct Producto {
    nombre: String,
    precio: f32,
    id: i32
}

impl Producto {
    fn new(nombre: String, precio: f32, id: i32) -> Producto {
        Producto { nombre, precio, id }
    }

    fn calcular_impuestos(&self, porc: f32) -> f32 {
        self.precio * (porc / 100.0)
    }

    fn aplicar_descuento(&self, porc: f32) -> f32 {
        self.precio * (porc / 100.0)
    }

    fn calcular_precio_total(&self, porc_imp: Option<f32>, porc_desc: Option<f32>) -> f32 {
        let mut precio = self.precio;

        if porc_desc.is_some() { precio = self.aplicar_descuento(porc_desc.unwrap()) }
        if porc_imp.is_some() { precio+= self.calcular_impuestos(porc_imp.unwrap()) }
        
        precio
    }

}

fn main() {

}