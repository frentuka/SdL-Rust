
/*
    1- Escribir un programa que defina una estructura Persona que tenga campos para:
        - nombre
        - edad
        - dirección (que puede ser nulo al momento de la creación de una persona).
        Para dicha estructura implemente los siguientes métodos:
        ➢ new: que pasando los parámetros correspondientes, crea una Persona y la retorna.
        ➢ imprimir: que imprime los datos de la persona sobre el mensaje ejecutado por ej: person.imprimir(),
            donde person es una variable del tipo Persona.
        ➢ obtener_edad: retorna la edad de la persona.
        ➢ actualizar_direccion(nueva_direccion)
 */

struct Persona {
    nombre:  String,
    edad: u16,
    direccion: Option<String>
}

impl Persona {
    fn new(nombre: String, edad: u16) -> Persona {
        Persona {nombre, edad, direccion: None}
    }

    fn actualizar_direccion(&mut self, direccion: String) {
        self.direccion = Some(direccion);
    }

    fn obtener_edad(&self) -> u16 {
        self.edad
    }

    fn imprimir(&self) {
        println!("Nombre: {}, Edad: {}", self.nombre, self.edad);
        if let Some(val) = &self.direccion { println!("Dirección: {}", val) }
    }

}

fn main() {

}

fn asd() {
    println!("asd");
}