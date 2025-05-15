
/*
    6- Escribir un programa que defina una estructura Estudiante que tenga campos para:
            nombre,
            número de identificación,
            las calificaciones de exámenes.
        De cada Examen se conoce:
            nombre de la materia,
            la nota.
        Para dichas estructuras implemente los siguientes métodos:
            ❖ Examen:
                ➢ new: que pasando los parámetros correspondientes, crea un Examen y lo retorna.
            ❖ Estudiante:
                ➢ new: que pasando los parámetros correspondientes, crea un Estudiante y lo retorna.
                ➢ obtener_promedio: retorna el promedio de las notas.
                ➢ obtener_calificacion_mas_alta: retorna la nota más alta.
                ➢ obtener_calificacion_mas_baja: retorna la nota más baja.
        Nota: Tenga en cuenta que el Estudiante puede tener entre 0 y n notas de examen.
 */
use std::cmp::min;

#[derive(Debug, Default, PartialEq, Clone, PartialOrd)]
struct Examen {
    materia: String,
    nota: f32
}

#[derive(Debug, Default, PartialEq, Clone, PartialOrd)]
struct Estudiante {
    nombre: String,
    id: u32,
    notas: Vec<Examen>
}

impl Examen {
    fn new(materia: String, nota: f32) -> Option<Examen> {
        if nota >= 0.0 { return Some(Examen { materia, nota }) }
        None
    }

}

/*
        Este informe debe incluir:
        Nombre e identificación del estudiante.,
        Cantidad total de exámenes rendidos.,
        Promedio general de notas.,
        Nota más alta y la materia correspondiente.,
        Nota más baja y la materia correspondiente.
 */
struct InformeAcademico {
    nombre: String,
    examenes_rendidos: u16,
    promedio_notas: Option<f32>,
    max_nota: Option<Examen>,
    min_nota: Option<Examen>
}

impl InformeAcademico {
    fn new(nombre: String, examenes_rendidos: u16, promedio_notas: Option<f32>, max_nota: Option<Examen>, min_nota: Option<Examen>) -> Option<InformeAcademico> {
        if examenes_rendidos != 0 && ( // si hay un examen rendido, todos los campos deben estar rellenos
            promedio_notas.is_none()
            || max_nota.is_none()
            || min_nota.is_none()) {
            return None;
        }

        Some(InformeAcademico { nombre, examenes_rendidos, promedio_notas, max_nota, min_nota })
    }
}


impl Estudiante {
    fn new(nombre: String, id: u32, notas: Vec<Examen>) -> Estudiante {
        Estudiante { nombre, id, notas }
    }

    // ➢ obtener_promedio: retorna el promedio de las notas.
    fn obtener_promedio(&self) -> Option<f32> {
        let cant_notas = self.notas.len();
        let mut suma_notas: u16 = 0;

        for examen in &self.notas {
            suma_notas+= examen.nota as u16;
        }

        if cant_notas != 0 { return Some(suma_notas as f32 / cant_notas as f32) }
        None
    }

    // ➢ obtener_calificacion_mas_alta: retorna la nota más alta.
    fn obtener_calificacion_mas_alta(&self) -> Option<f32> {
        self.notas.iter().max_by(|a, b| a.nota.total_cmp(&b.nota)).map(|val| val.nota)
    }

    // ➢ obtener_calificacion_mas_baja: retorna la nota más baja.
    fn obtener_calificacion_mas_baja(&self) -> Option<f32> {
        self.notas.iter().max_by(|a, b| b.nota.total_cmp(&a.nota)).map(|val| val.nota)
    }

    /*
        Deberán agregar una funcionalidad al ejercicio que permita retornar un informe detallado del rendimiento académico de un estudiante.

        Este informe debe incluir:
        Nombre e identificación del estudiante.,
        Cantidad total de exámenes rendidos.,
        Promedio general de notas.,
        Nota más alta y la materia correspondiente.,
        Nota más baja y la materia correspondiente.
     */

    /*
        struct InformeAcademico {
            nombre: String,
            examenes_rendidos: u16,
            promedio_notas: Option<f32>,
            max_nota: Option<Examen>,
            min_nota: Option<Examen>
        }
     */
    fn generar_informe(&self) -> Option<InformeAcademico> {
        let mut suma_total_notas: f32 = 0.0;
        let mut min_nota: Option<&Examen> = None;
        let mut max_nota: Option<&Examen> = None;

        for examen in &self.notas {
            if min_nota.is_none() { min_nota = Some(examen) }
            if max_nota.is_none() { max_nota = Some(examen) }

            if examen.nota < min_nota.unwrap().nota { min_nota = Some(examen); }
            if examen.nota < max_nota.unwrap().nota { max_nota = Some(examen); }

            suma_total_notas+= examen.nota;
        }

        let promedio_notas: Option<f32> = match self.notas.len() {
            0 => None,
            cant_notas => Some(suma_total_notas / (cant_notas as f32))
        };

        InformeAcademico::new(
            self.nombre.clone(),
            self.notas.len() as u16,
            promedio_notas,
            max_nota.cloned(),
            min_nota.cloned()
        )
    }

}

fn main() {

}

#[cfg(test)]
mod test {
    use crate::{Estudiante, Examen};

    #[test]
    fn test() {
        let examen1 = Examen::new("OC".to_string(), 4.0);
        let examen2 = Examen::new("Matematica 2".to_string(), 7.0);

        assert!(examen1.is_some(), "Examen1 debería ser algo");
        assert!(examen2.is_some(), "Examen2 debería ser algo");

        let nuevo_estudiante = Estudiante::new("jorgito".to_string(), 29, vec![examen1.unwrap(), examen2.unwrap()]);

        assert_eq!(nuevo_estudiante.nombre, "jorgito");
        assert_eq!(nuevo_estudiante.id, 29);
        assert_eq!(nuevo_estudiante.obtener_calificacion_mas_alta(), Some(7.0));
        assert_eq!(nuevo_estudiante.obtener_calificacion_mas_baja(), Some(4.0));
        assert_eq!(nuevo_estudiante.obtener_promedio(), Some(5.5));
    }

}