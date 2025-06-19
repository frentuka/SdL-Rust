use std::fmt;
use std::fmt::{format, write, Formatter};
use serde::{Deserialize, Deserializer, Serialize};
pub(crate) use crate::structs::fecha::Fecha;

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

impl fmt::Display for Mascota {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}) {}", self.edad, self.nombre)
    }
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