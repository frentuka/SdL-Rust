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
use crate::structs::{Atencion, Fecha, Mascota, Veterinaria};

impl Veterinaria {

    // ➔ crear una veterinaria.
    fn new(nombre: String, direccion: String, id: i32, cola: Option<VecDeque<Mascota>>, atenciones: Option<Vec<Atencion>>) -> Veterinaria {
        Veterinaria { nombre, direccion, id, cola: cola.unwrap_or_default(), atenciones: atenciones.unwrap_or_default() }
    }

    // ➔ agregar una nueva mascota a la cola de atención de la veterinaria.
    fn agregar_mascota(&mut self, mascota: Mascota) {
        self.cola.push_back(mascota);
    }

    // ➔ agregar una nueva mascota a la cola de atención pero que sea la siguiente en atender porque tiene la máxima prioridad.
    fn agregar_mascota_prioridad(&mut self, mascota: Mascota) {
        self.cola.push_front(mascota);
    }

    // ➔ atender la próxima mascota de la cola.
    fn atender_proxima_mascota(&mut self) -> Option<Mascota> {
        self.cola.pop_front()
    }

    // ➔ eliminar una mascota específica de la cola de atención dado que se retira.
    fn eliminar_mascota(&mut self, mascota: Mascota) {
        self.cola.retain(|m| *m != mascota);
    }
    
    // ➔ registrar una atención.
    fn registrar_atencion(&mut self, atencion: Atencion) {
        self.atenciones.push(atencion);
    }
    
    // ➔ buscar una atención dado el nombre de la mascota, el nombre del dueño y el teléfono.
    fn buscar_atencion(&self, nombre_mascota: String, nombre_dueno: String, telefono: u64) -> Option<&Atencion> {
        for atencion in &self.atenciones {
            if atencion.mascota.nombre == nombre_mascota
            && atencion.mascota.dueno.nombre == nombre_dueno
            && atencion.mascota.dueno.telefono == telefono 
                { return Some(atencion) }
        }
        
        None
    }
    
    // ➔ modificar el diagnóstico de una determinada atención.
    fn modificar_diagnostico_atencion(&mut self, atencion: &Atencion, nuevo_diagnostico: String) {
        for a in self.atenciones.iter_mut() {
            if a == atencion {
                a.diagnostico = nuevo_diagnostico.clone();
            }
        }
    }
    
    // ➔ modificar la fecha de la próxima visita de una determinada atención.
    fn modificar_fecha_atencion(&mut self, atencion: &Atencion, nueva_fecha: Fecha) {
        for a in self.atenciones.iter_mut() {
            if a == atencion {
                a.proxima_visita = nueva_fecha.clone();
            }
        }
    }
    
    // ➔ eliminar una determinada atención.
    fn eliminar_atencion(&mut self, atencion: &Atencion) {
        self.atenciones.retain(|m| m != atencion);
    }
}