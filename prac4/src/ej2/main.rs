
/*

2- Dado el siguiente struct:
    struct Persona<'a> {
        nombre:&'a str,
        apellido:&'a str,
        direccion:&'a str,
        ciudad:&'a str,
        salario:f64,
        edad:u8,
    }
a- Escriba una función que reciba un vector de personas y otro parámetro que indica un salario y retorna un listado de personas donde el salario es mayor al parámetro recibido.
b- Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad, y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro ciudad.
c- Escriba una función que reciba un vector de personas y un nombre de una ciudad y retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso contrario.
d- Escriba una función que reciba un vector de personas y un nombre de una ciudad y retorna true si al menos vive una persona en la ciudad pasada por parámetro,, false caso contrario.
e- Escriba una función que reciba un arreglo de personas y una persona y retorna true si la persona existe en el arreglo, false caso contrarioUNLP. Facultad de Informática.
Seminario de Lenguajes opción Rust Cursada 2023
f -Escriba una función que reciba un arreglo de personas y retorna un arreglo con las edades de las personas.
g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor salario y la persona con el mayor salario, en caso de que haya más de una persona en cada categoría desempatar por la edad más grande.

Nota: Implemente todos los métodos y traits que considere para resolver los ejercicios.
    Todos los ejercicios deben resolverse con iterator y closure.

 */

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
struct Persona {
    nombre: String,
    apellido: String,
    direccion: String,
    ciudad: String,
    salario: f64,
    edad: u8,
}

trait VecPersona {
    fn a_personas_salario_mayor(&self, num: f64) -> Option<Vec<&Persona>>;
    fn b_personas_mayores_edad_en_ciudad(&self, ciudad: &str, edad: u8) -> Vec<&Persona>;
    fn c_todos_viven_en_ciudad(&self, ciudad: &str) -> bool;
    fn d_alguien_vive_en_ciudad(&self, ciudad: &str) -> bool;
    fn e_persona_existe(&self, persona: &Persona) -> bool;
    fn f_listar_edades(&self) -> Vec<u8>;
    fn g_mayor_menor_salario(&self) -> Option<(&Persona, &Persona)>;
}

impl VecPersona for Vec<Persona> {
    fn a_personas_salario_mayor(&self, num: f64) -> Option<Vec<&Persona>> {
        if num < 0.0 { return None } // num debe ser un número positivo

        Some(
            self.iter().filter(
                |p|
                    p.salario > num
            ).collect()
        )
    }

    fn b_personas_mayores_edad_en_ciudad(&self, ciudad: &str, edad: u8) -> Vec<&Persona> {
        self.iter().filter(|p|
            p.ciudad == ciudad && p.edad > edad
        ).collect()
    }

    fn c_todos_viven_en_ciudad(&self, ciudad: &str) -> bool {
        self.iter().all(|p| p.ciudad == ciudad)
    }

    fn d_alguien_vive_en_ciudad(&self, ciudad: &str) -> bool {
        self.iter().any(|p| p.ciudad == ciudad)
    }

    fn e_persona_existe(&self, persona: &Persona) -> bool {
        self.iter().any(|p| p == persona)
    }

    fn f_listar_edades(&self) -> Vec<u8> {
        self.iter().map(|p| {
            p.edad
        }).collect()
    }

    fn g_mayor_menor_salario(&self) -> Option<(&Persona, &Persona)> {
        if self.len() < 2 { return None } // no min/max can be calculated with 1 or 0 elements

        let first_person = self.first();
        let first_person = first_person?;

        let mut res_index: (usize, usize) = (0, 0);
        let mut res: (&Persona, &Persona) = (first_person, first_person);

        self.iter().enumerate().for_each(
            | (i, p) | {
                if p.salario < res.0.salario {
                    res_index.0 = i;
                    res.0 = p;
                } else if p.salario > res.1.salario {
                    res_index.1 = i;
                    res.1 = p;
                } else if p.salario == res.0.salario && p.edad > res.0.edad {
                    res_index.0 = i;
                    res.0 = p;
                } else if p.salario == res.1.salario && p.edad > res.1.edad {
                    res_index.1 = i;
                    res.0 = p;
                }
            }
        );

        Some(res)
    }
}

fn main() { }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_personas_salario_mayor() {
        let personas = vec![
            Persona { nombre: "Alice".to_string(), apellido: "Smith".to_string(), direccion: "123 Main St".to_string(), ciudad: "Springfield".to_string(), salario: 50000.0, edad: 30 },
            Persona { nombre: "Bob".to_string(), apellido: "Johnson".to_string(), direccion: "456 Elm St".to_string(), ciudad: "Shelbyville".to_string(), salario: 60000.0, edad: 40 },
        ];
        let result = personas.a_personas_salario_mayor(55000.0);
        assert_eq!(result, Some(vec![&personas[1]]));

        let result = personas.a_personas_salario_mayor(-1.0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_b_personas_mayores_edad_en_ciudad() {
        let personas = vec![
            Persona { nombre: "Alice".to_string(), apellido: "Smith".to_string(), direccion: "123 Main St".to_string(), ciudad: "Springfield".to_string(), salario: 50000.0, edad: 30 },
            Persona { nombre: "Bob".to_string(), apellido: "Johnson".to_string(), direccion: "456 Elm St".to_string(), ciudad: "Springfield".to_string(), salario: 60000.0, edad: 40 },
        ];
        let result = personas.b_personas_mayores_edad_en_ciudad("Springfield", 35);
        assert_eq!(result, vec![&personas[1]]);
    }

    #[test]
    fn test_c_todos_viven_en_ciudad() {
        let personas = vec![
            Persona { nombre: "Alice".to_string(), apellido: "Smith".to_string(), direccion: "123 Main St".to_string(), ciudad: "Springfield".to_string(), salario: 50000.0, edad: 30 },
            Persona { nombre: "Bob".to_string(), apellido: "Johnson".to_string(), direccion: "456 Elm St".to_string(), ciudad: "Springfield".to_string(), salario: 60000.0, edad: 40 },
        ];
        assert!(personas.c_todos_viven_en_ciudad("Springfield"));
        assert!(!personas.c_todos_viven_en_ciudad("Shelbyville"));
    }

    #[test]
    fn test_d_alguien_vive_en_ciudad() {
        let personas = vec![
            Persona { nombre: "Alice".to_string(), apellido: "Smith".to_string(), direccion: "123 Main St".to_string(), ciudad: "Springfield".to_string(), salario: 50000.0, edad: 30 },
            Persona { nombre: "Bob".to_string(), apellido: "Johnson".to_string(), direccion: "456 Elm St".to_string(), ciudad: "Shelbyville".to_string(), salario: 60000.0, edad: 40 },
        ];

        assert!(personas.d_alguien_vive_en_ciudad("Springfield"));
        assert!(personas.d_alguien_vive_en_ciudad("Shelbyville"));
        assert!(!personas.d_alguien_vive_en_ciudad("Unknown City"));
    }

    #[test]
    fn test_e_persona_existe() {
        let personas = vec![
            Persona { nombre: "Alice".to_string(), apellido: "Smith".to_string(), direccion: "123 Main St".to_string(), ciudad: "Springfield".to_string(), salario: 50000.0, edad: 30 },
            Persona { nombre: "Bob".to_string(), apellido: "Johnson".to_string(), direccion: "456 Elm St".to_string(), ciudad: "Shelbyville".to_string(), salario: 60000.0, edad: 40 },
        ];

        let persona_existente = Persona { nombre: "Alice".to_string(), apellido: "Smith".to_string(), direccion: "123 Main St".to_string(), ciudad: "Springfield".to_string(), salario: 50000.0, edad: 30 };
        let persona_no_existente = Persona { nombre: "Charlie".to_string(), apellido: "Brown".to_string(), direccion: "789 Oak St".to_string(), ciudad: "Springfield".to_string(), salario: 70000.0, edad: 35 };

        assert!(personas.e_persona_existe(&persona_existente));
        assert!(!personas.e_persona_existe(&persona_no_existente));
    }

    #[test]
    fn test_f_listar_edades() {
        let personas = vec![
            Persona { nombre: "Alice".to_string(), apellido: "Smith".to_string(), direccion: "123 Main St".to_string(), ciudad: "Springfield".to_string(), salario: 50000.0, edad: 30 },
            Persona { nombre: "Bob".to_string(), apellido: "Johnson".to_string(), direccion: "456 Elm St".to_string(), ciudad: "Shelbyville".to_string(), salario: 60000.0, edad: 40 },
        ];

        let edades = personas.f_listar_edades();
        assert_eq!(edades, vec![30, 40]);
    }

    #[test]
    fn test_g_mayor_menor_salario() {
        let personas = vec![
            Persona { nombre: "Alice".to_string(), apellido: "Smith".to_string(), direccion: "123 Main St".to_string(), ciudad: "Springfield".to_string(), salario: 50000.0, edad: 30 },
            Persona { nombre: "Bob".to_string(), apellido: "Johnson".to_string(), direccion: "456 Elm St".to_string(), ciudad: "Shelbyville".to_string(), salario: 60000.0, edad: 40 },
            Persona { nombre: "Charlie".to_string(), apellido: "Brown".to_string(), direccion: "789 Oak St".to_string(), ciudad: "Springfield".to_string(), salario: 50000.0, edad: 35 },
        ];

        let result = personas.g_mayor_menor_salario();
        assert!(result.is_some());
        let (menor, mayor) = result.unwrap();
        assert_eq!(menor.salario, 50000.0);
        assert_eq!(mayor.salario, 60000.0);

        let personas = vec![
            Persona { nombre: "Alice".to_string(), apellido: "Smith".to_string(), direccion: "123 Main St".to_string(), ciudad: "Springfield".to_string(), salario: 50000.0, edad: 30 }
        ];

        assert_eq!(personas.g_mayor_menor_salario(), None);
    }
}