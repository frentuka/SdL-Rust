/*
    9.-Dada una cadena de veterinarias se desea implementar un sistema de atención de pacientes para cada veterinaria.
        De la veterinaria se conoce
            - nombre
            - la dirección
            - un id.
        Para la atención de mascotas se requiere administrar una cola de atención.
        De la mascota se conoce
            - nombre
            - edad
            - tipo de animal (perro, gato, caballo, otros)
            - su dueño.
        Del dueño se conoce
            - nombre
            - direccion
            - teléfono de contacto.
        Luego de la atención se desea tener un registro de las atenciones realizadas guardando:
            - datos de la mascota
            - el diagnóstico final
            - tratamiento
            - fecha de la próxima visita si es que se requiere.
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
        Nota: para la fecha utilice lo implementado en el punto 3.
 */
use std::collections::VecDeque;

// De la veterinaria se conoce
// - nombre
// - la dirección
// - un id.
pub struct Veterinaria {
    pub nombre: String,
    pub direccion: String,
    pub id: i32,
    pub cola: VecDeque<Mascota>,
    pub atenciones: Vec<Atencion>
}

#[derive(PartialEq)]
pub enum Animal {
    Perro, Gato, Caballo, Otros
}

// De la mascota se conoce
// - nombre
// - edad
// - tipo de animal (perro, gato, caballo, otros)
// - su dueño.
#[derive(PartialEq)]
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
#[derive(PartialEq)]
pub struct Dueno {
    pub nombre: String,
    pub direccion: String,
    pub telefono: u64,
}

// Luego de la atención se desea tener un registro de las atenciones realizadas guardando:
// - datos de la mascota
// - el diagnóstico final
// - tratamiento
// - fecha de la próxima visita si es que se requiere.
#[derive(PartialEq)]
pub struct Atencion {
    pub mascota: Mascota,
    pub diagnostico: String,
    pub tratamiento: String,
    pub proxima_visita: Fecha,
}

// Nota: para la fecha utilice lo implementado en el punto 3.
#[derive(PartialEq, Clone)]
pub struct Fecha {
    pub dia: u8,
    pub mes: u8,
    pub ano: i64
}