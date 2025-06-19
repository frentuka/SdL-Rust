use std::fs;
use error_proc_macro::Error;
use crate::structs::auto::Auto;

pub struct Concesionario<'a> {
    pub nombre: &'a str,
    pub direccion: &'a str,
    pub autos: Vec<Auto<'a>>
}

#[derive(Error, PartialEq)]
pub enum ErrorAgregarAuto {
    ConcesionarioLleno { capacidad: usize },
    ArchivoNoGuardado
}

#[derive(Error, PartialEq)]
pub enum ErrorEliminarAuto {
    ConcesionarioVacio,
    AutoInexistente,
    ArchivoNoGuardado
}

impl<'a> Concesionario<'a> {
    // ➢ new: que pasando los parámetros correspondientes, crea un ConcesionarioAuto y lo retorna.
    pub fn new(nombre: &'a str, direccion: &'a str, capacidad: usize) -> Self {
        Self { nombre, direccion, autos: Vec::with_capacity(capacidad) }
    }

    // c- Una vez hecho el punto anterior debe hacer que los autos de la concesionaria se
    // almacenen en un archivo en formato JSON. Agregue y modifique lo que considere necesario para que:
    //  - Al agregar un nuevo auto se abre el archivo de autos guardados y lo agregue a dicho archivo.
    //  - Eliminar un auto: al eliminar un auto se debe eliminar este del archivo.

    // ➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene
    //     sin superar la máxima cantidad para albergarlos
    //     y retorna true, en caso de que lo supere no lo agrega y retorna false.
    // returns usize -> Espacio disponible
    pub fn agregar_auto(&mut self, auto: Auto<'a>) -> Result<usize, ErrorAgregarAuto> {
        // a- Al agregar un auto si supera el límite de la concesionaria debe arrojar un error propio con un mensaje de contexto.
        if  self.autos.len() == self.autos.capacity() {
            return Err(ErrorAgregarAuto::ConcesionarioLleno { capacidad: self.autos.capacity() })
        }

        self.autos.push(auto);

        if !self.reescribir_json_autos() { return Err(ErrorAgregarAuto::ArchivoNoGuardado) }

        Ok(
            self.autos.capacity() - self.autos.len()
        )
    }

    // ➢ eliminar_auto(auto): elimina un auto de la lista de autos.
    pub fn eliminar_auto(&mut self, marca: &'a str, modelo: &'a str, ano: u16) -> Result<Auto, ErrorEliminarAuto> {
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

    // ➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
    pub fn buscar_auto(&self, marca: &'a str, modelo: &'a str, ano: u16) -> Option<&Auto> {
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
    use crate::structs::auto::Color;
    use super::*;

    fn concesionario<'a>(capacity: usize) -> Concesionario<'a> {
        Concesionario{
            nombre: "asd",
            direccion: "dire",
            autos: Vec::with_capacity(capacity)
        }
    }

    #[test]
    fn test_agregar() {
        let mut c = concesionario(1);

        let auto1 = Auto { marca: "Nissan", modelo: "March 1.6", ano: 2012, precio: 10.0, color: Color::Negro };
        let auto2 = Auto { marca: "Nissan2", modelo: "March 1.62", ano: 20122, precio: 10.2, color: Color::Azul };

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
    fn test_eliminar() {
        let mut c = concesionario(1);

        let auto1 = Auto { marca: "Nissan", modelo: "March 1.6", ano: 2012, precio: 10.0, color: Color::Negro };

        let res_agregar_1 = c.agregar_auto(auto1.clone());

        let res_eliminar_1 = c.eliminar_auto("asd", "asdasdanoexiste", 9999);

        match res_eliminar_1 {
            Ok(_) => { panic!("Debería fallar") }
            Err(err) => { assert_eq!(err, ErrorEliminarAuto::AutoInexistente, "el auto a eliminar no existe") }
        }

        let res_eliminar_2 = c.eliminar_auto(auto1.marca, auto1.modelo, auto1.ano);

        assert!(res_eliminar_2.is_ok(), "No debería fallar, el auto existe");

        let res_eliminar_3 = c.eliminar_auto(auto1.marca, auto1.modelo, auto1.ano);

        match res_eliminar_3 {
            Ok(_) => {}
            Err(err) => { assert_eq!(err, ErrorEliminarAuto::ConcesionarioVacio, "el concesionario está vacío") }
        }
    }

}