
//
// fecha.rs
//

use std::collections::VecDeque;
use std::{fmt, fs, io};
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};

const NOMBRE_MESES: [&str; 12] = ["Enero", "Febrero", "Marzo", "Abril",
    "Mayo", "Junio", "Julio", "Agosto",
    "Septiembre", "Octubre", "Noviembre", "Diciembre"];
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Default)]
pub struct Fecha {
    pub(crate) time_since_epoch: u64,
}

//
// atencion.rs
//

// Luego de la atención se desea tener un registro de las atenciones realizadas guardando:
// - datos de la mascota
// - el diagnóstico final
// - tratamiento
// - fecha de la próxima visita si es que se requiere.

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Atencion {
    pub mascota: Mascota,
    pub diagnostico: String,
    pub tratamiento: String,
    pub proxima_visita: Fecha,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub enum Animal {
    Perro, Gato, Caballo,
    #[default] Otros
}

// De la mascota se conoce
// - nombre
// - edad
// - tipo de animal (perro, gato, caballo, otros)
// - su dueño.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Mascota {
    pub nombre: String,
    pub edad: u16,
    pub animal: Animal,
    pub dueno: Dueno,
}

// Del dueño se conoce
// - nombre
// - direccion
// - teléfono de contacto.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Dueno {
    pub nombre: String,
    pub direccion: String,
    pub telefono: u64,
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

//
// veterinaria.rs
//

const BASE_FOLDER: &str = "";

// De la veterinaria se conoce
// - nombre
// - la dirección
// - un id.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Veterinaria {
    pub nombre: String,
    pub direccion: String,
    pub id: u64,
    pub cola: VecDeque<Mascota>,
    pub atenciones: Vec<Atencion>
}

//
// results ArchivoAtenciones
//

#[derive(Debug, PartialEq)]
pub enum ResultArchivoAtenciones {
    Read{ atenciones: Vec<Atencion> },
    Written{ json: String },
    IOError,
    SerializeError,
    DeserializeError
}

//
// errores veterinaria
//

#[derive(Debug, PartialEq)]
pub enum ErrorNewVeterinaria {
    VecColaCapacidad0,
    VecAtencionesCapacidad0,
    ArchivoAtenciones(ResultArchivoAtenciones)
}

#[derive(Debug, PartialEq)]
pub enum ResultAgregarMascota {
    Exito,
    ColaLlena{ capacity: usize },
    ArchivoAtenciones(ResultArchivoAtenciones)
}

#[derive(Debug, PartialEq)]
pub enum ResultRemoverMascota {
    Exito (Mascota),
    ColaVacia,
    MascotaInexistente { nombre_mascota: String, nombre_dueno: String },
    ArchivoAtenciones(ResultArchivoAtenciones),
}

#[derive(Debug, PartialEq)]
pub enum ResultAgregarAtencion {
    Exito,
    ArchivoAtenciones(ResultArchivoAtenciones)
}

#[derive(Debug, PartialEq)]
pub enum ResultRemoverAtencion {
    Exito(Atencion),
    VectorVacio,
    AtencionInexistente,
    ArchivoAtenciones(ResultArchivoAtenciones)
}


//
// impl Veterinaria
//

impl Veterinaria {

    // ➔ crear una veterinaria.
    pub fn new(
        nombre: &str,
        direccion: &str,
        id: u64,
        cola: Option<VecDeque<Mascota>>,
        atenciones: Option<Vec<Atencion>>
    ) -> Result<Self, ErrorNewVeterinaria> {
        let cola = if let Some(cola) = cola {
            if cola.capacity() == 0 { return Err(ErrorNewVeterinaria::VecColaCapacidad0) }
            cola
        } else { VecDeque::new() };

        // si atenciones some(val) -> crear un archivo que contenga val
        // si atenciones none -> intentar abrir el archivo y colocar su información en self.atenciones

        let atenciones = if let Some(atenciones) = atenciones {
            if atenciones.capacity() == 0 { return Err(ErrorNewVeterinaria::VecAtencionesCapacidad0)  }
            sobreescribir_archivo_atenciones(nombre, &atenciones);
            atenciones
        } else {
            // leer o crear
            match leer_archivo_atenciones(nombre) {
                ResultArchivoAtenciones::Read { atenciones } => { atenciones },
                error => return Err(ErrorNewVeterinaria::ArchivoAtenciones(error))
            }
        };

        Ok(
            Self { nombre: nombre.to_string(), direccion: direccion.to_string(), id, cola, atenciones }
        )
    }

    // ➔ agregar una nueva mascota a la cola de atención de la veterinaria.
    pub fn agregar_mascota(&mut self, mascota: Mascota) -> ResultAgregarMascota {
        if self.cola.len() == self.cola.capacity() { return ResultAgregarMascota::ColaLlena { capacity: self.cola.capacity() } }
        self.cola.push_back(mascota);
        ResultAgregarMascota::Exito
    }

    // ➔ agregar una nueva mascota a la cola de atención pero que sea la siguiente en atender porque tiene la máxima prioridad.
    pub fn agregar_mascota_prioridad(&mut self, mascota: Mascota) -> ResultAgregarMascota {
        if self.cola.len() == self.cola.capacity() { return ResultAgregarMascota::ColaLlena { capacity: self.cola.capacity() } }
        self.cola.push_front(mascota);
        ResultAgregarMascota::Exito
    }

    // ➔ atender la próxima mascota de la cola.
    pub fn atender_proxima_mascota(&mut self) -> ResultRemoverMascota {
        if let Some(mascota) = self.cola.pop_front() { ResultRemoverMascota::Exito(mascota) }
        else { ResultRemoverMascota::ColaVacia }
    }

    // ➔ eliminar una mascota específica de la cola de atención dado que se retira.
    pub fn eliminar_mascota(&mut self, nombre_mascota: &str, nombre_dueno: &str) -> ResultRemoverMascota {
        let index = if let Some(index) = self.cola.iter().position(|m| m.nombre == nombre_mascota && m.dueno.nombre == nombre_dueno ) { index }
        else { return ResultRemoverMascota::MascotaInexistente { nombre_mascota: nombre_mascota.to_string(), nombre_dueno: nombre_dueno.to_string() } };

        if let Some(mascota) = self.cola.remove(index) {
            ResultRemoverMascota::Exito(mascota)
        } else {
            ResultRemoverMascota::MascotaInexistente { nombre_mascota: self.nombre.to_string(), nombre_dueno: nombre_dueno.to_string() }
        }
    }

    // ➔ registrar una atención.
    pub fn registrar_atencion(&mut self, atencion: Atencion) -> ResultAgregarAtencion {
        self.atenciones.push(atencion);

        match sobreescribir_archivo_atenciones(&self.nombre, &self.atenciones) {
            ResultArchivoAtenciones::Written { .. } => { ResultAgregarAtencion::Exito },
            x => ResultAgregarAtencion::ArchivoAtenciones(x)
        }
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
    pub fn buscar_atencion(&self, mascota_nombre: &str, dueno_nombre: &str, telefono: u64) -> Option<&Atencion> {
        self.atenciones.iter().find(|atencion| {
            atencion.mascota.nombre == mascota_nombre
                && atencion.mascota.dueno.nombre == dueno_nombre
                && atencion.mascota.dueno.telefono == telefono
        })
    }

    // pub fn buscar_atencion_mut(&mut self, mascota_nombre: String, dueno_nombre: String, telefono: u64, ) -> Option<&mut Atencion> {
    //     self.atenciones.iter_mut().find(|atencion| {
    //         atencion.mascota.nombre == mascota_nombre
    //             && atencion.mascota.dueno.nombre == dueno_nombre
    //             && atencion.mascota.dueno.telefono == telefono
    //     }) // cómo hago para editar el archivo cuando se haya actualizado un &mut Atencion?
    //         // cómo hago para saber cuándo se edita con un préstamo mutable de un elemento?
    //          // cómo hago para saber cuándo finaliza el préstamo mutable?
    // }

    pub fn buscar_atencion_mut(&mut self, mascota_nombre: String, dueno_nombre: String, telefono: u64) -> Option<& mut Atencion> {
        self.atenciones.iter_mut().find(|atencion|
            atencion.mascota.nombre == mascota_nombre
                && atencion.mascota.dueno.nombre == dueno_nombre
                && atencion.mascota.dueno.telefono == telefono)
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
    pub fn eliminar_atencion (&mut self, nombre_mascota: &str, nombre_dueno: &str, diagnostico: &str) -> ResultRemoverAtencion {
        if self.atenciones.is_empty() {
            return ResultRemoverAtencion::VectorVacio
        }

        if let Some(index) = self.atenciones.iter().position( |a|
            a.mascota.nombre == nombre_mascota
                && a.mascota.dueno.nombre == nombre_dueno
                && a.diagnostico == diagnostico) {

            let atencion = self.atenciones.remove(index);

            match sobreescribir_archivo_atenciones(&self.nombre, &self.atenciones) {
                ResultArchivoAtenciones::Written { .. } => { ResultRemoverAtencion::Exito(atencion) },
                x => ResultRemoverAtencion::ArchivoAtenciones(x)
            }
        } else {
            ResultRemoverAtencion::AtencionInexistente
        }
    }
}

//
//  atenciones file
//

fn archivo_atenciones_filepath(nombre_vet: &str) -> String {
    format!("{}veterinaria_{}_atenciones.json", BASE_FOLDER, nombre_vet)
}

//

fn sobreescribir_archivo_atenciones(nombre_vet: &str, atenciones: &Vec<Atencion>) -> ResultArchivoAtenciones {
    // crear y/o escribir
    // presupongo que la imposibilidad de persistir como archivo es un error semi-catastrófico
    // semi-catastrófico == no runtime panic, pero se aborta la creación de la veterinaria
    let json_data = match serde_json::to_string_pretty(atenciones) {
        Ok(data) => { data }
        Err(error) => { return ResultArchivoAtenciones::SerializeError }
    };

    // escribir
    match fs::write(archivo_atenciones_filepath(nombre_vet), json_data.clone()) {
        Ok(_) => {  },
        Err(error) => {
            return ResultArchivoAtenciones::IOError
        }
    };

    // éxito
    ResultArchivoAtenciones::Written { json: json_data }
}

//

fn leer_archivo_atenciones(nombre_vet: &str) -> ResultArchivoAtenciones {
    // leer
    let mut file = match File::open(archivo_atenciones_filepath(nombre_vet)) {
        Ok(file) => { file }
        Err(error) => { return ResultArchivoAtenciones::IOError }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(error) => return ResultArchivoAtenciones::IOError
    };

    let json_value: serde_json::Value = match serde_json::from_str(&contents) {
        Ok(value) => value,
        Err(error) => return ResultArchivoAtenciones::DeserializeError,
    };

    let atenciones: Vec<Atencion> = match serde_json::from_value(json_value) {
        Ok(atenciones) => atenciones,
        Err(error) => return ResultArchivoAtenciones::DeserializeError,
    };

    ResultArchivoAtenciones::Read { atenciones }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use super::*;

    fn veterinaria_de_pepe(capacidad: usize) -> Veterinaria {
        // Crear dueños
        let dueno1 = Dueno {
            nombre: "Juan Pérez".to_string(),
            direccion: "Calle 123".to_string(),
            telefono: 1234567890,
        };
        let dueno2 = Dueno {
            nombre: "María Gómez".to_string(),
            direccion: "Avenida 456".to_string(),
            telefono: 9876543210,
        };
        let dueno3 = Dueno {
            nombre: "Carlos López".to_string(),
            direccion: "Carrera 789".to_string(),
            telefono: 5555555555,
        };

        // Crear mascotas para la cola
        let mascota1 = Mascota {
            nombre: "Max".to_string(),
            edad: 3,
            animal: Animal::Perro,
            dueno: dueno1,
        };
        let mascota2 = Mascota {
            nombre: "Luna".to_string(),
            edad: 2,
            animal: Animal::Gato,
            dueno: dueno2,
        };
        let mascota3 = Mascota {
            nombre: "Tormenta".to_string(),
            edad: 5,
            animal: Animal::Caballo,
            dueno: dueno3,
        };

        // Crear fechas para atenciones
        let fecha1 = Fecha { time_since_epoch: 15 };
        let fecha2 = Fecha { time_since_epoch: 20 };
        let fecha3 = Fecha { time_since_epoch: 30 };

        // Crear atenciones
        let atencion1 = Atencion {
            mascota: mascota1.clone(),
            diagnostico: "Resfriado leve".to_string(),
            tratamiento: "Antibióticos".to_string(),
            proxima_visita: fecha1,
        };
        let atencion2 = Atencion {
            mascota: mascota2.clone(),
            diagnostico: "Infección ocular".to_string(),
            tratamiento: "Gotas".to_string(),
            proxima_visita: fecha2,
        };
        let atencion3 = Atencion {
            mascota: mascota3.clone(),
            diagnostico: "Cojeo".to_string(),
            tratamiento: "Reposo".to_string(),
            proxima_visita: fecha3,
        };

        let mut vec_cola: VecDeque<Mascota> = VecDeque::with_capacity(capacidad);
        vec_cola.push_back(mascota1);
        vec_cola.push_back(mascota2);
        vec_cola.push_back(mascota3);

        // Crear la veterinaria con cola y atenciones
        match Veterinaria::new (
            "Pepe's Pet Shop",
            "Calle Principal 100",
            1,
            Some(vec_cola),
            Some(vec![atencion1, atencion2, atencion3]),
        ) {
            Ok(vet) => { vet }
            Err(err) => { panic!("error: {:?}", err) }
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
        let mut veterinaria = veterinaria_de_pepe(100);

        // Buscar

        let atencion = veterinaria.buscar_atencion_mut("Max".to_string(), "Juan Pérez".to_string(), 1234567890);
        assert!(atencion.is_some(), "La atención no puede no existir");
        let atencion = atencion.unwrap();

        atencion.modificar_diagnostico_atencion("jijodebu en los jijolines jijox".to_string());
        assert_eq!(atencion.diagnostico, "jijodebu en los jijolines jijox");

        atencion.modificar_fecha_atencion(Fecha { time_since_epoch: 10 });
        assert_eq!(atencion.proxima_visita, Fecha { time_since_epoch: 10 });

        // registrar

        let atencion1 = Atencion {
            mascota: Mascota {
                nombre: "asdasdasdad".to_string(),
                edad: 0,
                animal: Default::default(),
                dueno: Default::default(),
            },
            diagnostico: String::from("Resfriado leve"),
            tratamiento: String::from("Antibióticos"),
            proxima_visita: Fecha { time_since_epoch: 100 },
        };

        let res = veterinaria.registrar_atencion(atencion1.clone());
        assert_eq!(res, ResultAgregarAtencion::Exito);

        let res = veterinaria.buscar_atencion(atencion1.mascota.nombre.as_str(), atencion1.mascota.dueno.nombre.as_str(), atencion1.mascota.dueno.telefono);
        assert_eq!(res, Some(&atencion1));

        let res = veterinaria.eliminar_atencion(atencion1.mascota.nombre.as_str(), atencion1.mascota.dueno.nombre.as_str(), atencion1.diagnostico.as_str());
        assert_eq!(res, ResultRemoverAtencion::Exito(atencion1.clone()));

        let res = veterinaria.eliminar_atencion(atencion1.mascota.nombre.as_str(), atencion1.mascota.dueno.nombre.as_str(), atencion1.diagnostico.as_str());
        assert_eq!(res, ResultRemoverAtencion::AtencionInexistente);

        let mut veterinaria = Veterinaria::default();
        let res = veterinaria.eliminar_atencion(atencion1.mascota.nombre.as_str(), atencion1.mascota.dueno.nombre.as_str(), atencion1.diagnostico.as_str());
        assert_eq!(res, ResultRemoverAtencion::VectorVacio);

        let res = veterinaria.atender_proxima_mascota();
        assert_eq!(res, ResultRemoverMascota::ColaVacia);

        let res = veterinaria.eliminar_mascota("asd", "asd");
        assert_eq!(res, ResultRemoverMascota::MascotaInexistente { nombre_mascota: "asd".to_string(), nombre_dueno: "asd".to_string() })
    }

    #[test]
    fn test_agregar_eliminar() {
        let mut veterinaria = veterinaria_de_pepe(10);

        let dueno1 = Dueno {
            nombre: "Enrique Ibañez".to_string(),
            direccion: "Calle 437".to_string(),
            telefono: 1234567890,
        };

        let mascota1 = Mascota {
            nombre: "Rodolfoasdasda".to_string(),
            edad: 72,
            animal: Animal::Caballo,
            dueno: dueno1,
        };

        match veterinaria.agregar_mascota(mascota1.clone()) {
            ResultAgregarMascota::Exito => {},
            ResultAgregarMascota::ColaLlena { capacity } => { panic!("No deberia estar llena. Capacidad: {}", capacity) },
            ResultAgregarMascota::ArchivoAtenciones(aa) => { panic!("e? {:?}", aa) }
        }

        println!("{:?}", veterinaria.cola);
        assert_eq!(veterinaria.cola.len(), 4, "Deberían haber 4 mascotas en total.");

        veterinaria.agregar_mascota_prioridad(mascota1.clone());
        let prox_mascota = veterinaria.atender_proxima_mascota();

        let mascota = match prox_mascota {
            ResultRemoverMascota::Exito(mascota) => { mascota },
            _ => panic!("deberia existir")
        };

        assert_eq!(mascota, mascota1, "La primer mascota en la fila debería ser la misma mascota que se agregó con prioridad");

        println!("{:?}", veterinaria.eliminar_mascota(mascota1.nombre.as_str(), mascota1.dueno.nombre.as_str()));

        assert_eq!(veterinaria.cola.len(), 3, "La lista de mascotas debería haber vuelto a su estado original (3 items).");
    }

    #[test]
    fn test_json() {
        veterinaria_de_pepe(10); // creará el .json

        match Veterinaria::new( // cargará el .json
                                "Pepe's Pet Shop",
                                "asd",
                                1,
                                None,
                                None
        ) {
            Ok(vet) => {
                assert_eq!(vet.atenciones.len(), 3, "deberían ser 3 según el archivo");

                for atencion in vet.atenciones {
                    println!("{} ({}): {}", atencion.mascota.nombre, atencion.mascota.edad, atencion.diagnostico)
                }
            }
            Err(err) => { panic!("error new veterinaria: {:?}", err) }
        }
    }
}