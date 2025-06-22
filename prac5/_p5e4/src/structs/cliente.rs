use serde::{Deserialize, Serialize};

/// Del cliente se conoce:
///     el nombre,
///     teléfono
///     y dirección de correo electrónico.
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, Debug)]
pub struct Cliente {
    pub id: u32,
    pub nombre: String,
    pub telefono: String,
    pub email: String,
}

impl Cliente {
    pub fn new(id: u32, nombre: String, telefono: String, email: String) -> Cliente {
        Cliente { id, nombre, telefono, email }
    }
}