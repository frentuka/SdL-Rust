
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

#[derive(Debug, Default, PartialEq, Clone, PartialOrd)]
struct Examen {
    materia: String,
    nota: u8
}

#[derive(Debug, Default, PartialEq, Clone, PartialOrd)]
struct Estudiante {
    nombre: String,
    id: u32,
    notas: Vec<Examen>
}

impl Examen {
    fn new(materia: String, nota: u8) -> Examen {
        Examen { materia, nota }
    }

}

impl Estudiante {
    fn new(nombre: String, id: u32, notas: Vec<Examen>) -> Estudiante {
        Estudiante { nombre, id, notas }
    }

    // ➢ obtener_promedio: retorna el promedio de las notas.
    fn obtener_promedio(&self) -> f32 {
        let cant_notas = self.notas.len();
        let mut suma_notas: u16 = 0;

        for examen in &self.notas {
            suma_notas+= examen.nota as u16;
        }

        if cant_notas != 0 { return suma_notas as f32 / cant_notas as f32 }
        0.0
    }

    // ➢ obtener_calificacion_mas_alta: retorna la nota más alta.
    fn obtener_calificacion_mas_alta(&self) -> u8 {
        match self.notas.iter().max_by(|a, b| a.nota.cmp(&b.nota)) {
            Some(val) => val.nota,
            None => 0
        }
    }

    // ➢ obtener_calificacion_mas_baja: retorna la nota más baja.
    fn obtener_calificacion_mas_baja(&self) -> u8 {
        match self.notas.iter().max_by(|a, b| b.nota.cmp(&a.nota)) {
            Some(val) => val.nota,
            None => 0
        }
    }

}

fn main() {

}

#[cfg(test)]
mod test {

    #[test]
    fn main () {

        // 

    }


}