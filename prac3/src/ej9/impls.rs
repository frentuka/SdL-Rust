/*
Dado todo lo mencionado anteriormente implemente los métodos para realizar las siguientes acciones:
    ➔ crear una veterinaria.
    ➔ agregar una nueva mascota a la cola de atención de la veterinaria.
    ➔ agregar una nueva mascota a la cola de atención pero que sea la siguiente en atender porque tiene la máxima prioridad.
    ➔ atender la próxima mascota de la cola.
    ➔ eliminar una mascota específica de la cola de atención dado que se retira.
    ➔ registrar una atención.
    ➔ buscar una atención dado el nombre de la mascota, el nombre del dueño y el teléfono.
    ➔ modificar el diagnóstico de una determinada atención.
    ➔ modificar la fecha de la próxima visita de una determinada atención.
    ➔ eliminar una determinada atención.
 */
use std::collections::VecDeque;
pub(crate) use crate::structs::{Atencion, Fecha, Mascota, Veterinaria};

impl Veterinaria {

    // ➔ crear una veterinaria.
    pub fn new(nombre: String, direccion: String, id: i32, cola: Option<VecDeque<Mascota>>, atenciones: Option<Vec<Atencion>>) -> Veterinaria {
        Veterinaria { nombre, direccion, id, cola: cola.unwrap_or_default(), atenciones: atenciones.unwrap_or_default() }
    }

    // ➔ agregar una nueva mascota a la cola de atención de la veterinaria.
    pub fn agregar_mascota(&mut self, mascota: Mascota) {
        self.cola.push_back(mascota);
    }

    // ➔ agregar una nueva mascota a la cola de atención pero que sea la siguiente en atender porque tiene la máxima prioridad.
    pub fn agregar_mascota_prioridad(&mut self, mascota: Mascota) {
        self.cola.push_front(mascota);
    }

    // ➔ atender la próxima mascota de la cola.
    pub fn atender_proxima_mascota(&mut self) -> Option<Mascota> {
        self.cola.pop_front()
    }

    // ➔ eliminar una mascota específica de la cola de atención dado que se retira.
    pub fn eliminar_mascota(&mut self, mascota: Mascota) {
        self.cola.retain(|m| *m != mascota);
    }
    
    // ➔ registrar una atención.
    pub fn registrar_atencion(&mut self, atencion: Atencion) {
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
    ) -> Option<&mut Atencion> {
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

impl Atencion {

    // ➔ modificar el diagnóstico de una determinada atención.
    pub fn modificar_diagnostico_atencion(&mut self, nuevo_diagnostico: String) {
        self.diagnostico = nuevo_diagnostico;
    }

    // ➔ modificar la fecha de la próxima visita de una determinada atención.
    pub fn modificar_fecha_atencion(&mut self, nueva_fecha: Fecha) {
        self.proxima_visita = nueva_fecha;
    }
    
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use crate::structs::{Animal, Atencion, Dueno, Fecha, Mascota, Veterinaria};

    fn veterinaria_de_pepe() -> Veterinaria {
        // Crear dueños
        let dueno1 = Dueno {
            nombre: String::from("Juan Pérez"),
            direccion: String::from("Calle 123"),
            telefono: 1234567890,
        };
        let dueno2 = Dueno {
            nombre: String::from("María Gómez"),
            direccion: String::from("Avenida 456"),
            telefono: 9876543210,
        };
        let dueno3 = Dueno {
            nombre: String::from("Carlos López"),
            direccion: String::from("Carrera 789"),
            telefono: 5555555555,
        };

        // Crear mascotas para la cola
        let mascota1 = Mascota {
            nombre: String::from("Max"),
            edad: 3,
            animal: Animal::Perro,
            dueno: dueno1,
        };
        let mascota2 = Mascota {
            nombre: String::from("Luna"),
            edad: 2,
            animal: Animal::Gato,
            dueno: dueno2,
        };
        let mascota3 = Mascota {
            nombre: String::from("Tormenta"),
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
            diagnostico: String::from("Resfriado leve"),
            tratamiento: String::from("Antibióticos"),
            proxima_visita: fecha1,
        };
        let atencion2 = Atencion {
            mascota: mascota2.clone(),
            diagnostico: String::from("Infección ocular"),
            tratamiento: String::from("Gotas"),
            proxima_visita: fecha2,
        };
        let atencion3 = Atencion {
            mascota: mascota3.clone(),
            diagnostico: String::from("Cojeo"),
            tratamiento: String::from("Reposo"),
            proxima_visita: fecha3,
        };

        // Crear la veterinaria con cola y atenciones
        Veterinaria {
            nombre: String::from("Pepe's Pet Shop"),
            direccion: String::from("Calle Principal 100"),
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

        // Verificar contenido
        println!("Veterinaria: {}", veterinaria.nombre);
        println!("Cola: {:?}", veterinaria.cola);
        println!("Atenciones: {:?}", veterinaria.atenciones);

        // Buscar

        let atencion = veterinaria.buscar_atencion_mut("Max".to_string(), "Juan Pérez".to_string(), 1234567890);
        assert!(atencion.is_some(), "La atención no puede no existir");
        let atencion = atencion.unwrap();

        atencion.modificar_diagnostico_atencion("jijodebu en los jijolines jijox".to_string());

        println!("Atenciones: {:?}", veterinaria.atenciones);
    }

    #[test]
    fn test_agregar_eliminar() {
        let mut veterinaria = veterinaria_de_pepe();

        let dueno1 = Dueno {
            nombre: String::from("Enrique Ibañez"),
            direccion: String::from("Calle 437"),
            telefono: 1234567890,
        };

        let mascota1 = Mascota {
            nombre: String::from("Rodolfo"),
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