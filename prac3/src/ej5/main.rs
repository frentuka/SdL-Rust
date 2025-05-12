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
    fn new(nombre: String, precio: f32, id: i32) -> Result<Producto, String> {
        if precio < 0.0 { return Err("Un precio no puede ser negativo.".to_string()) }
        Ok(Producto { nombre, precio, id })
    }

    fn calcular_impuestos(&self, porc: f32) -> f64 {
        self.precio as f64 * porc as f64 / 100.0
    }

    fn aplicar_descuento(&self, porc: f32) -> f64 {
        self.precio as f64 * (1.0 - porc as f64 / 100.0)
    }

    fn calcular_precio_total(&self, porc_imp: Option<f32>, porc_desc: Option<f32>) -> f64 {
        let mut precio: f64 = self.precio as f64;

        if porc_desc.is_some() { precio-= self.aplicar_descuento(porc_desc.unwrap()) }
        if porc_imp.is_some() { precio+= self.calcular_impuestos(porc_imp.unwrap()) }
        
        precio
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use crate::Producto;

    #[test]
    fn test_limites() {
        let producto = Producto::new("asd".to_string(), f32::MAX, i32::MAX).unwrap();

        // should not panic
        let precio_total_1 = producto.calcular_precio_total(Some(f32::MAX), Some(f32::MAX));
        let precio_total_2 = producto.calcular_precio_total(Some(f32::MIN), Some(f32::MIN));
    }

}