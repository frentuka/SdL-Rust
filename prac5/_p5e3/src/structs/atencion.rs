pub(crate) use crate::structs::fecha::Fecha;

// Luego de la atención se desea tener un registro de las atenciones realizadas guardando:
// - datos de la mascota
// - el diagnóstico final
// - tratamiento
// - fecha de la próxima visita si es que se requiere.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Atencion<'a> {
    pub mascota: Mascota<'a>,
    pub diagnostico: &'a str,
    pub tratamiento: &'a str,
    pub proxima_visita: Fecha,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum Animal {
    Perro, Gato, Caballo,
    #[default] Otros
}

// De la mascota se conoce
// - nombre
// - edad
// - tipo de animal (perro, gato, caballo, otros)
// - su dueño.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Mascota<'a> {
    pub nombre: &'a str,
    pub edad: u16,
    pub animal: Animal,
    pub dueno: Dueno<'a>,
}

// Del dueño se conoce
// - nombre
// - direccion
// - teléfono de contacto.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Dueno<'a> {
    pub nombre: &'a str,
    pub direccion: &'a str,
    pub telefono: u64,
}

impl<'a> Atencion<'a> {
    // ➔ modificar el diagnóstico de una determinada atención.
    pub fn modificar_diagnostico_atencion(&mut self, nuevo_diagnostico: &'a str) {
        self.diagnostico = nuevo_diagnostico;
    }

    // ➔ modificar la fecha de la próxima visita de una determinada atención.
    pub fn modificar_fecha_atencion(&mut self, nueva_fecha: Fecha) {
        self.proxima_visita = nueva_fecha;
    }
}