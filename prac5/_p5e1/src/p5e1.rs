//
// auto.rs
//

use std::fs;
use error_proc_macro::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Color {
    Rojo, Verde, Azul, Amarillo, Blanco, Negro
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Auto {
    pub marca: String,
    pub modelo: String,
    pub ano: u16,
    pub precio: f64,
    pub color: Color
}

#[derive(Error)]
pub enum ErrorNewAuto {
    InvalidYear{ year: u16 },
    InvalidPrice{ price: f64 },
}

impl Auto {
    // ➢ new: que pasando los parámetros correspondientes, crea un Auto y lo retorna.
    pub fn new(marca: &str, modelo: &str, ano: u16, precio: f64, color: Color) -> Result<Self, ErrorNewAuto> {
        if ano < 1886 { return Err(ErrorNewAuto::InvalidYear{ year: ano }) }
        if precio < 0.0 || precio.is_nan() || !precio.is_finite() { return Err(ErrorNewAuto::InvalidPrice{ price: precio }) }

        Ok(Self { marca: marca.to_string(), modelo: modelo.to_string(), ano, precio, color })
    }

    // ➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
    pub fn calcular_precio(&self) -> f64 {
        // ■ si es de color primario le aplica un recargo del 25%, sino le aplica un descuento del 10%.
        let recargo_color = match self.color {
            Color::Rojo | Color::Azul | Color::Amarillo => self.precio * 0.25,
            _ => self.precio * -0.1
        };

        // ■ si la marca es BMW le aplica un recargo del 15%
        let recargo_bmw = if self.marca == "BMW" { self.precio * 0.15 } else { 0.0 };

        // ■ si el año es menor a 2000 le aplica un descuento del 5%.
        let descuento_ano = if self.ano < 2000 { self.precio * 0.05 } else { 0.0 };

        self.precio + recargo_color + recargo_bmw - descuento_ano
    }
}

//
// concesionario.rs
//

pub struct Concesionario {
    pub nombre: String,
    pub direccion: String,
    pub autos: Vec<Auto>,
    pub capacidad: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ErrorNewConcesionario {
    ErrorCapacidadInsuficiente(String)
}

#[derive(Debug, PartialEq, Clone)]
pub enum ErrorAgregarAuto {
    ConcesionarioLleno { capacidad: usize },
    ArchivoNoGuardado
}

#[derive(Debug, PartialEq, Clone)]
pub enum ErrorEliminarAuto {
    ConcesionarioVacio,
    AutoInexistente,
    ArchivoNoGuardado
}

impl Concesionario {
    // ➢ new: que pasando los parámetros correspondientes, crea un ConcesionarioAuto y lo retorna.
    pub fn new(nombre: &str, direccion: &str, capacidad: usize) -> Result<Self, ErrorNewConcesionario> {
        // si el archivo existe, cargarlo
        let op_read = fs::read_to_string("autos.json");
        let vec_autos: Vec<Auto> = if let Ok(data) = op_read {
            let new_autos = serde_json::from_str::<Vec<Auto>>(data.as_str()).unwrap_or_default();

            if new_autos.len() > capacidad {
                return Err(ErrorNewConcesionario::ErrorCapacidadInsuficiente("La cantidad de autos cargada desde el archivo es mayor a la capacidad del concesionario".to_string()))
            }

            new_autos
        } else {
            Vec::with_capacity(capacidad)
        };

        Ok(Self { nombre: nombre.to_string(), direccion: direccion.to_string(), autos: vec_autos, capacidad })
    }

    fn reescribir_json_autos(&self) -> bool {
        // c. reescribir el archivo con la información del vector de autos
        match serde_json::to_string_pretty(&self.autos) {
            Ok(res) => {
                if fs::write("autos.json", res).is_err() {
                    return false
                }
                
                true
            }
            Err(_) => { false }
        }
    }

    // c- Una vez hecho el punto anterior debe hacer que los autos de la concesionaria se
    // almacenen en un archivo en formato JSON. Agregue y modifique lo que considere necesario para que:
    //  - Al agregar un nuevo auto se abre el archivo de autos guardados y lo agregue a dicho archivo.
    //  - Eliminar un auto: al eliminar un auto se debe eliminar este del archivo.

    // ➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene
    //     sin superar la máxima cantidad para albergarlos
    //     y retorna true, en caso de que lo supere no lo agrega y retorna false.
    // returns usize -> Espacio disponible
    pub fn agregar_auto(&mut self, auto: Auto) -> Result<usize, ErrorAgregarAuto> {
        // a- Al agregar un auto si supera el límite de la concesionaria debe arrojar un error propio con un mensaje de contexto.
        if  self.autos.len() >= self.capacidad {
            return Err(ErrorAgregarAuto::ConcesionarioLleno { capacidad: self.capacidad })
        }

        self.autos.push(auto);

        if !self.reescribir_json_autos() { return Err(ErrorAgregarAuto::ArchivoNoGuardado) }

        Ok(
            self.autos.capacity() - self.autos.len()
        )
    }

    // ➢ eliminar_auto(auto): elimina un auto de la lista de autos.
    pub fn eliminar_auto(&mut self, marca: &str, modelo: &str, ano: u16) -> Result<Auto, ErrorEliminarAuto> {
        if self.autos.is_empty() { return Err(ErrorEliminarAuto::ConcesionarioVacio) }

        let mut found_index = 0;
        let mut found = false;
        for (index, auto) in self.autos.iter().enumerate() {
            if auto.marca == marca && auto.modelo == modelo && auto.ano == ano {
                found_index = index; found = true; break;
            }
        }

        if !found {
            return Err(ErrorEliminarAuto::AutoInexistente)
        }

        let auto = self.autos.remove(found_index);

        if !self.reescribir_json_autos() { Err(ErrorEliminarAuto::ArchivoNoGuardado) }
        else { Ok(auto) }
    }

    // ➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
    pub fn buscar_auto(&self, marca: &str, modelo: &str, ano: u16) -> Option<&Auto> {
        for auto in &self.autos {
            if auto.marca == marca && auto.modelo == modelo && auto.ano == ano {
                return Some(auto)
            }
        }

        None
    }
}

/*
    b- Haga todos los tests correspondientes para probar en profundidad los métodos que agregan un auto y eliminan un auto de la concesionaria,
        obteniendo el mayor porcentaje de coverage sobre el código que realiza las operaciones.
 */

#[cfg(test)]
mod tests {
    use Color;
    use super::*;

    fn concesionario(capacidad: usize) -> Concesionario {
        Concesionario {
            nombre: "asd".to_string(),
            direccion: "dire".to_string(),
            autos: Vec::with_capacity(capacidad),
            capacidad
        }
    }

    #[test]
    fn test_auto_new() {
        let auto = Auto::new("Nissan", "March 1.6", 2012, 10.0, Color::Negro);
        assert!(auto.is_ok());
        let auto = auto.unwrap();
        assert_eq!(auto.marca, "Nissan");
        assert_eq!(auto.modelo, "March 1.6");
    }

    #[test]
    fn test_calcular_precio_color_primario() {
        let auto = Auto { marca: "Nissan".to_string(), modelo: "March".to_string(), ano: 2010, precio: 100.0, color: Color::Rojo };
        // Rojo: +25%
        assert_eq!(auto.calcular_precio(), 125.0);

        let auto = Auto { marca: "Nissan".to_string(), modelo: "March".to_string(), ano: 2010, precio: 100.0, color: Color::Azul };
        // Azul: +25%
        assert_eq!(auto.calcular_precio(), 125.0);

        let auto = Auto { marca: "Nissan".to_string(), modelo: "March".to_string(), ano: 2010, precio: 100.0, color: Color::Amarillo };
        // Amarillo: +25%
        assert_eq!(auto.calcular_precio(), 125.0);
    }

    #[test]
    fn test_calcular_precio_color_no_primario() {
        let auto = Auto { marca: "Nissan".to_string(), modelo: "March".to_string(), ano: 2010, precio: 100.0, color: Color::Negro };
        // Negro: -10%
        assert_eq!(auto.calcular_precio(), 90.0);

        let auto = Auto { marca: "Nissan".to_string(), modelo: "March".to_string(), ano: 2010, precio: 100.0, color: Color::Blanco };
        // Blanco: -10%
        assert_eq!(auto.calcular_precio(), 90.0);

        let auto = Auto { marca: "Nissan".to_string(), modelo: "March".to_string(), ano: 2010, precio: 100.0, color: Color::Verde };
        // Verde: -10%
        assert_eq!(auto.calcular_precio(), 90.0);
    }

    #[test]
    fn test_calcular_precio_bmw() {
        let auto = Auto { marca: "BMW".to_string(), modelo: "X5".to_string(), ano: 2010, precio: 100.0, color: Color::Rojo };
        // Rojo: +25%, BMW: +15%
        assert_eq!(auto.calcular_precio(), 140.0);

        let auto = Auto { marca: "BMW".to_string(), modelo: "X5".to_string(), ano: 2010, precio: 100.0, color: Color::Negro };
        // Negro: -10%, BMW: +15%
        assert_eq!(auto.calcular_precio(), 105.0);
    }

    #[test]
    fn test_calcular_precio_ano_menor_2000() {
        let auto = Auto { marca: "Nissan".to_string(), modelo: "March".to_string(), ano: 1999, precio: 100.0, color: Color::Rojo };
        // Rojo: +25%, año < 2000: -5%
        assert_eq!(auto.calcular_precio(), 120.0);

        let auto = Auto { marca: "BMW".to_string(), modelo: "X5".to_string(), ano: 1999, precio: 100.0, color: Color::Negro };
        // Negro: -10%, BMW: +15%, año < 2000: -5%
        assert_eq!(auto.calcular_precio(), 100.0);
    }

    #[test]
    fn test_buscar_auto_found_and_not_found() {
        let mut c = concesionario(1000);
        let auto1 = Auto { marca: "Nissan".to_string(), modelo: "March".to_string(), ano: 2010, precio: 100.0, color: Color::Negro };
        let auto2 = Auto { marca: "BMW".to_string(), modelo: "X5".to_string(), ano: 2015, precio: 200.0, color: Color::Rojo };
        c.agregar_auto(auto1.clone()).unwrap();
        c.agregar_auto(auto2.clone()).unwrap();

        // Found
        let found = c.buscar_auto("Nissan", "March", 2010);
        assert!(found.is_some());
        assert_eq!(found.unwrap(), &auto1);

        let found = c.buscar_auto("BMW", "X5", 2015);
        assert!(found.is_some());
        assert_eq!(found.unwrap(), &auto2);

        // Not found
        let not_found = c.buscar_auto("Ford", "Fiesta", 2010);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_concesionario_agregar() {
        let mut c = concesionario(1);

        let auto1 = Auto { marca: "Nissan".to_string(), modelo: "March 1.6".to_string(), ano: 2012, precio: 10.0, color: Color::Negro };
        let auto2 = Auto { marca: "Nissan2".to_string(), modelo: "March 1.62".to_string(), ano: 20122, precio: 10.2, color: Color::Azul };

        let res_agregar_1 = c.agregar_auto(auto1);
        let res_agregar_2 = c.agregar_auto(auto2);

        assert!(res_agregar_1.is_ok(), "No debe causar error: concesionario tiene un auto de capacidad.");
        assert!(res_agregar_2.is_err(), "Debe causar error: concesionario tiene sólo un auto de capacidad.");

        let res_agregar_1 = res_agregar_1.unwrap();
        let res_agregar_2 = res_agregar_2.unwrap_err();

        assert_eq!(res_agregar_1, 0, "No debe quedar espacio para más autos.");
        assert_eq!(res_agregar_2, ErrorAgregarAuto::ConcesionarioLleno { capacidad: 1 });
    }

    #[test]
    fn test_concesionario_eliminar() {
        let mut c = concesionario(1);

        let auto1 = Auto { marca: "Nissan".to_string(), modelo: "March 1.6".to_string(), ano: 2012, precio: 10.0, color: Color::Negro };

        let res_agregar_1 = c.agregar_auto(auto1.clone());

        let res_eliminar_1 = c.eliminar_auto("asd", "asdasdanoexiste", 9999);

        match res_eliminar_1 {
            Ok(_) => { panic!("Debería fallar") }
            Err(err) => { assert_eq!(err, ErrorEliminarAuto::AutoInexistente, "el auto a eliminar no existe") }
        }

        let res_eliminar_2 = c.eliminar_auto(auto1.marca.as_str(), auto1.modelo.as_str(), auto1.ano);

        assert!(res_eliminar_2.is_ok(), "No debería fallar, el auto existe");

        let res_eliminar_3 = c.eliminar_auto(auto1.marca.as_str(), auto1.modelo.as_str(), auto1.ano);

        match res_eliminar_3 {
            Ok(_) => {}
            Err(err) => { assert_eq!(err, ErrorEliminarAuto::ConcesionarioVacio, "el concesionario está vacío") }
        }
    }
}