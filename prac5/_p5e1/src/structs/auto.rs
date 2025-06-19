use error_proc_macro::Error;
use serde::{Serialize, Serializer};

#[derive(Serialize, PartialEq, Clone)]
pub enum Color {
    Rojo, Verde, Azul, Amarillo, Blanco, Negro
}

#[derive(Serialize, PartialEq, Clone)]
pub struct Auto<'a> {
    pub marca: &'a str,
    pub modelo: &'a str,
    pub ano: u16,
    pub precio: f64,
    pub color: Color
}

#[derive(Error)]
pub enum ErrorNewAuto {
    InvalidYear{ year: u16 },
    InvalidPrice{ price: f64 },
}

impl<'a> Auto<'a> {
    // ➢ new: que pasando los parámetros correspondientes, crea un Auto y lo retorna.
    pub fn new(marca: &'a str, modelo: &'a str, ano: u16, precio: f64, color: Color) -> Result<Self, ErrorNewAuto> {
        if ano < 1886 { return Err(ErrorNewAuto::InvalidYear{ year: ano }) }
        if precio < 0.0 || precio.is_nan() || !precio.is_finite() { return Err(ErrorNewAuto::InvalidPrice{ price: precio }) }
        
        Ok(Self { marca, modelo, ano, precio, color })
    }

    // ➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
    pub fn calcular_precio(&self) -> f64 {
        let precio = self.precio;

        // ■ si es de color primario le aplica un recargo del 25%, sino le aplica un descuento del 10%.
        let recargo_color = match self.color {
            Color::Rojo | Color::Azul | Color::Amarillo => precio * 0.25,
            _ => precio * -0.1
        };

        // ■ si la marca es BMW le aplica un recargo del 15%
        let recargo_bmw = if self.marca == "BMW" { precio * 0.15 } else { 0.0 };

        // ■ si el año es menor a 2000 le aplica un descuento del 5%.
        let descuento_ano = if self.ano < 2000 { precio * 0.05 } else { 0.0 };

        precio + recargo_color + recargo_bmw - descuento_ano
    }
}