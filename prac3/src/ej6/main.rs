const CALIFICACION_MAXIMA: f32 = 10.0;

/// ### Examen
/// `materia: String` - Nombre de la materia del examen<br>
/// `nota: f32` - Calificación de un Estudiante en dicho examen
#[derive(Debug, PartialEq)]
struct Examen<'e> {
    materia: &'e str,
    nota: f32
}

/// ### Estudiante
/// `nombre: String` - Nombre del estudiante<br>
/// `id: u32` - ID del estudiante<br>
/// `notas: Vec<Examen>` - Vector de exámenes que rindió el estudiante
struct Estudiante<'a, 'e> {
    nombre: &'a str,
    id: u32,
    notas: Vec<Examen<'e>>
}

/// ### InformeAcademico<br>
/// `nombre: String` - Nombre e identificación del estudiante<br>
/// `examenes_rendidos: u16` - Cantidad total de exámenes rendidos<br>
/// `promedio_notas: Option<f32>` - Promedio general de notas<br>
/// `max_nota: Option<&Examen>` - Nota mas alta y la materia correspondiente<br>
/// `min_nota: Option<&Examen>` - Nota más baja y la materia correspondiente
struct InformeAcademico<'a, 'e> {
    nombre: &'a str,
    id: u32,
    examenes_rendidos: u16,
    promedio_notas: Option<f32>,
    max_nota: Option<&'a Examen<'e>>,
    min_nota: Option<&'a Examen<'e>>
}

impl<'e> Examen<'e> {
    /// ### new(materia, nota) -> Option\<Examen\>
    /// Crea y devuelve una nueva instancia de Examen si los datos proporcionados son válidos
    /// 
    /// #### Recibe:
    /// `materia: String` - Nombre de la materia del examen<br>
    /// `nota: f32` - Nota del estudiante en dicho examen
    /// 
    /// #### Devuelve:
    /// `Some(Examen)` - Examen que contiene toda la información recibida<br>
    /// `None` - Los datos recibidos no son válidos
    /// 
    /// #### Excepciones inválidas:
    /// `nota < 0.0` - La nota debe ser un número positivo<br>
    /// `nota > CALIFICACION_MAXIMA` - La nota debe ser menor o igual a la calificación máxima permitida
    fn new(materia: &str, nota: f32) -> Option<Examen> {
        if (0.0..=CALIFICACION_MAXIMA).contains(&nota) { return Some(Examen { materia, nota }) }
        None
    }
}

impl<'a, 'e> InformeAcademico<'a, 'e> {
    
    /// ### fn new(nombre, examenes_rendidos, promedio_notas, max_nota, min_nota) -> Option\<InformeAcademico\>
    /// Crea y devuelve una nuvea instancia de InformeAcademico si los datos proporcionados son válidos
    /// 
    /// #### Recibe:
    /// `nombre` - Nombre del estudiante<br>
    /// `examenes_rendidos` - Cantidad de exámenes rendidos por el estudiante<br>
    /// `promedio_notas` - Promedio de exámenes rendidos<br>
    /// `max_nota` - Examen con mayor nota del estudiante<br>
    /// `min_nota` - Examen con menor nota del estudiante
    /// 
    /// #### Devuelve:
    /// `Some(InformeAcademico)` - Informe académico que contiene toda la información recibida<br>
    /// `None` - Hubo una incongruencia en los datos recibidos
    /// 
    /// #### Incongruencias
    /// `examenes_rendidos == 0` - Pero alguno de los datos (promedio, max_nota, min_nota) presenta información<br>
    /// `examenes_rendidos != 0` - Pero alguno de los datos (promedio, max_not, min_nota) no presenta información
    fn new(nombre: &'a str,
           id: u32,
           examenes_rendidos: u16,
           promedio_notas: Option<f32>,
           max_nota: Option<&'e Examen>,
           min_nota: Option<&'e Examen>
    ) -> Option<InformeAcademico<'a, 'e>> {
        
        // Si hay al menos un examen rendido todos los campos deben ser Some()
        if examenes_rendidos != 0 && (
            promedio_notas.is_none()
            || max_nota.is_none()
            || min_nota.is_none()) {
            return None;
        }

        // Caso opuesto. Si no hay exámenes rendidos todos los campos deben ser None
        if examenes_rendidos == 0 && (
            promedio_notas.is_some()
            || max_nota.is_some()
            || min_nota.is_some()) {
            return None;
        }

        Some(InformeAcademico { nombre, id, examenes_rendidos, promedio_notas, max_nota, min_nota })
    }
}

impl<'a, 'e> Estudiante<'a, 'e> {
    /// ### new(nombre, id, notas) -> Estudiante
    /// Crea una nueva instancia de Estudiante
    fn new(nombre: &'a str, id: u32, notas: Vec<Examen<'e>>) -> Estudiante<'a, 'e> {
        Estudiante { nombre, id, notas }
    }

    /// ### fn obtener_calificacion_promedio() -> Option\<f32\>
    /// Devuelve la mayor calificacion del estudiante
    ///
    /// #### Devuelve:
    /// `Some(f32)` - La calificación promedio del estudiante<br>
    /// `None` - El estudiante no tiene calificaciones
    fn obtener_calificacion_promedio(&self) -> Option<f32> {
        let cant_notas = self.notas.len();
        let mut suma_notas: u16 = 0;

        for examen in &self.notas {
            suma_notas+= examen.nota as u16;
        }

        if cant_notas != 0 { return Some(suma_notas as f32 / cant_notas as f32) }
        None
    }

    /// ### fn obtener_calificacion_mas_alta() -> Option\<f32\>
    /// Devuelve la mayor calificacion del estudiante
    ///
    /// #### Devuelve:
    /// `Some(f32)` - La mayor calificación del estudiante<br>
    /// `None` - El estudiante no tiene calificaciones
    fn obtener_calificacion_mas_alta(&self) -> Option<f32> {
        self.notas.iter().max_by(|a, b| a.nota.total_cmp(&b.nota)).map(|val| val.nota)
    }

    /// ### fn obtener_calificacion_mas_baja() -> Option\<f32\>
    /// Devuelve la menor calificacion del estudiante
    ///
    /// #### Devuelve:
    /// `Some(f32)` - La menor calificación del estudiante<br>
    /// `None` - El estudiante no tiene calificaciones
    fn obtener_calificacion_mas_baja(&self) -> Option<f32> {
        self.notas.iter().max_by(|a, b| b.nota.total_cmp(&a.nota)).map(|val| val.nota)
    }

    /// ### fn generar_informe() -> Option<InformeAcademico>
    /// Procesa los datos del estudiante y devuelve un informe acorde.
    ///
    /// #### Devuelve:
    /// `Some(InformeAcademico)` - Informe académico del alumno<br>
    /// `None` - Incongruencias no permitieron fabricar el informe académico
    fn generar_informe(&self) -> Option<InformeAcademico> {
        let mut suma_total_notas: f32 = 0.0;
        let mut min_nota: Option<&Examen> = None;
        let mut max_nota: Option<&Examen> = None;

        /*
            Sería posible utilizar las funciones ya existentes de la struct Estudiante:
                fn obtener_calificacion_promedio() -> Option<f32>
                fn obtener_calificacion_mas_alta() -> Option<f32>
                fn obtener_calificacion_mas_baja() -> Option<f32>
            Pero en cada una se realiza una consulta individual al vector de exámenes del estudiante,
            por lo que estaría recorrieno la misma información 3 veces, lo cual es absolutamente innecesario
            entonces me resulta una mucho mejor idea prescindir de todas estas funciones.
         */

        for examen in &self.notas {
            match min_nota {
                Some(val) => if examen.nota < val.nota { min_nota = Some(examen) }
                None => min_nota = Some(examen)
            }

            match max_nota {
                Some(val) => if examen.nota > val.nota { max_nota = Some(examen) }
                None => max_nota = Some(examen)
            }

            suma_total_notas+= examen.nota;
        }

        let promedio_notas: Option<f32> = match self.notas.len() {
            0 => None,
            cant_notas => Some(suma_total_notas / (cant_notas as f32))
        };

        // Sólo puede "fallar" (devolver None) si:
        //      self.notas != 0 && (promedio_notas != Some(f32) || max_nota != Some(Examen) || min_nota != Some(Examen))
        //   || self.notas == 0 && (promedio_notas == Some(f32) || max_nota == Some(Examen) || min_nota == Some(Examen))
        // lo cual, creo, es imposible.
        InformeAcademico::new(
            self.nombre,
            self.id,
            self.notas.len() as u16,
            promedio_notas,
            max_nota,
            min_nota
        )
    }
}

fn main() { }

#[cfg(test)]
mod test {
    use crate::{Estudiante, Examen};

    fn estudiante<'a, 'e>() -> Estudiante<'a, 'e> {
        let examen1 = Examen::new("SdL Rust", 4.0);
        let examen2 = Examen::new("AyED", 5.0);
        let examen3 = Examen::new("FOD", 6.0);
        let examen4 = Examen::new("Matematica 3", 7.0);

        assert!(examen1.is_some(), "Examen1 debería brindar Some(Examen)");
        assert!(examen2.is_some(), "Examen2 debería brindar Some(Examen)");
        assert!(examen3.is_some(), "Examen3 debería brindar Some(Examen)");
        assert!(examen4.is_some(), "Examen4 debería brindar Some(Examen)");

        Estudiante::new("jorgito", 13548, vec![
            examen1.unwrap(),
            examen2.unwrap(),
            examen3.unwrap(),
            examen4.unwrap()])
    }

    #[test]
    fn test_max_min_prom() {
        let estudiante = estudiante();

        assert_eq!(estudiante.obtener_calificacion_mas_alta(), Some(7.0));
        assert_eq!(estudiante.obtener_calificacion_mas_baja(), Some(4.0));
        assert_eq!(estudiante.obtener_calificacion_promedio(), Some(5.5));
    }

    #[test]
    fn test_informe_academico_some() {
        let estudiante = estudiante();
        let informe = estudiante.generar_informe();

        assert!(informe.is_some(), "Informe debería brindar Some(InformeAcademico)");

        let informe = informe.unwrap();

        let nombre_estudiante = informe.nombre;
        let id_estudiante = informe.id;
        let examenes_rendidos = informe.examenes_rendidos;

        let min_nota = informe.min_nota;
        let max_nota = informe.max_nota;
        let promedio = informe.promedio_notas;

        assert_eq!(nombre_estudiante, "jorgito".to_string(), "El nombre se debe preservar");
        assert_eq!(id_estudiante, 13548, "El nombre se debe preservar");
        assert_eq!(examenes_rendidos, 4, "Se rindieron 4 exámenes");

        assert!(min_nota.is_some(), "Debería existir nota mínima");
        assert!(max_nota.is_some(), "Debería existir nota máxima");
        assert!(promedio.is_some(), "Debería existir promedio");

        assert_eq!(min_nota.unwrap().nota, 4.0, "La nota mínima debería ser 4.0");
        assert_eq!(max_nota.unwrap().nota, 7.0, "La nota máxima debería ser 7.0");
        assert_eq!(promedio.unwrap(), 5.5, "La nota promedio debería ser 5.5");
    }

    #[test]
    fn test_informe_academico_none() {
        let estudiante = Estudiante::new("jorgito", 29, Vec::new());
        let informe = estudiante.generar_informe();

        assert!(informe.is_some(), "Informe debería brindar Some(InformeAcademico)");

        let informe = informe.unwrap();

        assert_eq!(informe.examenes_rendidos, 0, "No deberían existir exámenes rendidos");
        assert_eq!(informe.max_nota, None, "No debería existir nota mínima");
        assert_eq!(informe.min_nota, None, "No debería existir nota máxima");
        assert_eq!(informe.promedio_notas, None, "No debería existir nota promedio");
    }
}