use std::collections::VecDeque;
use crate::structs::atencion::{Atencion, Mascota};

// De la veterinaria se conoce
// - nombre
// - la dirección
// - un id.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Veterinaria<'a> {
    pub nombre: &'a str,
    pub direccion: &'a str,
    pub id: i32,
    pub cola: VecDeque<Mascota<'a>>,
    pub atenciones: Vec<Atencion<'a>>
}

impl<'a> Veterinaria<'a> {

    // ➔ crear una veterinaria.
    pub fn new(nombre: &'a str, direccion: &'a str, id: i32, cola: Option<VecDeque<Mascota<'a>>>, atenciones: Option<Vec<Atencion<'a>>>) -> Self {
        Self { nombre, direccion, id, cola: cola.unwrap_or_default(), atenciones: atenciones.unwrap_or_default() }
    }

    // ➔ agregar una nueva mascota a la cola de atención de la veterinaria.
    pub fn agregar_mascota(&mut self, mascota: Mascota<'a>) {
        self.cola.push_back(mascota);
    }

    // ➔ agregar una nueva mascota a la cola de atención pero que sea la siguiente en atender porque tiene la máxima prioridad.
    pub fn agregar_mascota_prioridad(&mut self, mascota: Mascota<'a>) {
        self.cola.push_front(mascota);
    }

    // ➔ atender la próxima mascota de la cola.
    pub fn atender_proxima_mascota(&mut self) -> Option<Mascota> {
        self.cola.pop_front()
    }

    // ➔ eliminar una mascota específica de la cola de atención dado que se retira.
    pub fn eliminar_mascota(&mut self, mascota: Mascota<'a>) {
        self.cola.retain(|m| *m != mascota);
    }

    // ➔ registrar una atención.
    pub fn registrar_atencion(&mut self, atencion: Atencion<'a>) {
        self.atenciones.push(atencion);
    }

    /// Searches for an Atencion by Mascota's name and Dueno's details.
    ///
    /// # Arguments
    /// * `mascota_nombre` - The name of the Mascota.
    /// * `dueno_nombre` - The name of the Dueno.
    /// * `telefono` - The Dueno's phone number.
    ///
    /// # Returns
    /// * `Some(&Atencion)` - If an Atencion is found.
    /// * `None` - If no match is found.
    pub fn buscar_atencion(
        &self,
        mascota_nombre: String,
        dueno_nombre: String,
        telefono: u64,
    ) -> Option<&Atencion> {
        self.atenciones.iter().find(|atencion| {
            atencion.mascota.nombre == mascota_nombre
                && atencion.mascota.dueno.nombre == dueno_nombre
                && atencion.mascota.dueno.telefono == telefono
        })
    }

    /// Searches for an Atencion by Mascota's name and Dueno's details.
    ///
    /// # Arguments
    /// * `mascota_nombre` - The name of the Mascota.
    /// * `dueno_nombre` - The name of the Dueno.
    /// * `telefono` - The Dueno's phone number.
    ///
    /// # Returns
    /// * `Some(&mut Atencion)` - If an Atencion is found.
    /// * `None` - If no match is found.
    pub fn buscar_atencion_mut(
        &mut self,
        mascota_nombre: String,
        dueno_nombre: String,
        telefono: u64,
    ) -> Option<&mut Atencion<'a>> {
        self.atenciones.iter_mut().find(|atencion| {
            atencion.mascota.nombre == mascota_nombre
                && atencion.mascota.dueno.nombre == dueno_nombre
                && atencion.mascota.dueno.telefono == telefono
        })
    }

    /// Deletes an Atencion record from the Veterinaria records
    ///
    /// # Arguments
    /// * `atencion` - A reference to the Atencion to delete
    ///
    /// # Returns
    /// `Some(Atencion)` - The deleted Atencion
    /// `None` - If no match is found
    ///
    /// # Notes
    /// Requires `PartialEq` on Atencion for comparison
    pub fn eliminar_atencion(
        &mut self,
        atencion: &Atencion
    ) -> Option<Atencion> {
        if let Some(index) = self.atenciones.iter().position(|a| a == atencion) {
            return Some(self.atenciones.remove(index));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use crate::structs::atencion::{Animal, Dueno, Fecha};
    use super::*;

    fn veterinaria_de_pepe<'a>() -> Veterinaria<'a> {
        // Crear dueños
        let dueno1 = Dueno {
            nombre: "Juan Pérez",
            direccion: "Calle 123",
            telefono: 1234567890,
        };
        let dueno2 = Dueno {
            nombre: "María Gómez",
            direccion: "Avenida 456",
            telefono: 9876543210,
        };
        let dueno3 = Dueno {
            nombre: "Carlos López",
            direccion: "Carrera 789",
            telefono: 5555555555,
        };

        // Crear mascotas para la cola
        let mascota1 = Mascota {
            nombre: "Max",
            edad: 3,
            animal: Animal::Perro,
            dueno: dueno1,
        };
        let mascota2 = Mascota {
            nombre: "Luna",
            edad: 2,
            animal: Animal::Gato,
            dueno: dueno2,
        };
        let mascota3 = Mascota {
            nombre: "Tormenta",
            edad: 5,
            animal: Animal::Caballo,
            dueno: dueno3,
        };

        // Crear fechas para atenciones
        let fecha1 = Fecha { dia: 15, mes: 10, ano: 2023 };
        let fecha2 = Fecha { dia: 20, mes: 10, ano: 2023 };
        let fecha3 = Fecha { dia: 25, mes: 10, ano: 2023 };

        // Crear atenciones
        let atencion1 = Atencion {
            mascota: mascota1.clone(),
            diagnostico: "Resfriado leve",
            tratamiento: "Antibióticos",
            proxima_visita: fecha1,
        };
        let atencion2 = Atencion {
            mascota: mascota2.clone(),
            diagnostico: "Infección ocular",
            tratamiento: "Gotas",
            proxima_visita: fecha2,
        };
        let atencion3 = Atencion {
            mascota: mascota3.clone(),
            diagnostico: "Cojeo",
            tratamiento: "Reposo",
            proxima_visita: fecha3,
        };

        // Crear la veterinaria con cola y atenciones
        Veterinaria {
            nombre: "Pepe's Pet Shop",
            direccion: "Calle Principal 100",
            id: 1,
            cola: VecDeque::from([mascota1, mascota2, mascota3]),
            atenciones: vec![atencion1, atencion2, atencion3],
        }
    }

    /*
        let dueno1 = Dueno {
            nombre: String::from("Juan Pérez"),
            direccion: String::from("Calle 123"),
            telefono: 1234567890,
        };

        let mascota1 = Mascota {
            nombre: String::from("Max"),
            edad: 3,
            animal: Animal::Perro,
            dueno: dueno1,
        };

        let fecha1 = Fecha { dia: 15, mes: 10, ano: 2023 };

        let atencion1 = Atencion {
            mascota: mascota1.clone(),
            diagnostico: String::from("Resfriado leve"),
            tratamiento: String::from("Antibióticos"),
            proxima_visita: fecha1,
        };

        Veterinaria {
            nombre: String::from("Pepe's Pet Shop"),
            direccion: String::from("Calle Principal 100"),
            id: 1,
            cola: VecDeque::from([mascota1, mascota2, mascota3]),
            atenciones: vec![atencion1, atencion2, atencion3],
        }
     */

    #[test]
    fn test_atencion() {
        let mut veterinaria = veterinaria_de_pepe();

        // Verificar visualmente el contenido
        println!("Veterinaria: {}", veterinaria.nombre);
        println!("Cola: {:?}", veterinaria.cola);
        println!("Atenciones: {:?}", veterinaria.atenciones);

        // Buscar

        let atencion = veterinaria.buscar_atencion_mut("Max".to_string(), "Juan Pérez".to_string(), 1234567890);
        assert!(atencion.is_some(), "La atención no puede no existir");
        let atencion = atencion.unwrap();

        atencion.modificar_diagnostico_atencion("jijodebu en los jijolines jijox");

        println!("Atenciones: {:?}", veterinaria.atenciones);
    }

    #[test]
    fn test_agregar_eliminar() {
        let mut veterinaria = veterinaria_de_pepe();

        let dueno1 = Dueno {
            nombre: "Enrique Ibañez",
            direccion: "Calle 437",
            telefono: 1234567890,
        };

        let mascota1 = Mascota {
            nombre: "Rodolfo",
            edad: 7,
            animal: Animal::Caballo,
            dueno: dueno1,
        };

        veterinaria.agregar_mascota(mascota1.clone());

        println!("{:?}", veterinaria.cola);
        assert_eq!(veterinaria.cola.len(), 4, "Deberían haber 4 mascotas en total.");

        veterinaria.agregar_mascota_prioridad(mascota1.clone());
        let prox_mascota = veterinaria.atender_proxima_mascota();
        assert!(prox_mascota.is_some(), "Debe existir una próxima mascota");
        assert_eq!(prox_mascota.unwrap(), mascota1, "La primer mascota en la fila debería ser la misma mascota que se agregó con prioridad");

        println!("{:?}", veterinaria.eliminar_mascota(mascota1));

        assert_eq!(veterinaria.cola.len(), 3, "La lista de mascotas debería haber vuelto a su estado original (3 items).");
    }
}