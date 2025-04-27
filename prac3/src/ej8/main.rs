
/*
    8- Defina la estructura Cancion con campos para
        - título
        - artista
        - el género.
    El género puede ser:
        - rock
        - pop
        - rap
        - jazz
        - otros.
    Luego modele una playlist.
    La playlist está compuesta por una lista de canciones y un nombre
    y se permiten hacer las siguientes acciones sobre ella:
        ➔ agregar canción.
        ➔ eliminar canción.
        ➔ mover canción // mueve la canción a una determinada posición de la playlist.
        ➔ buscar canción por nombre.
        ➔ obtener las canciones de un determinado género.
        ➔ obtener las canciones de un determinado artista.
        ➔ modificar título de la playlist.
        ➔ eliminar todas las canciones.
 */
use std::cmp::PartialEq;

#[derive(PartialEq, Clone, Debug)]
enum Genero {
    Rock, Pop, Rap, Jazz, Otros
}

#[derive(PartialEq, Clone, Debug)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

struct Playlist {
    nombre: String,
    canciones: Vec<Cancion>
}

impl Cancion {
    fn new(titulo: String, artista: String, genero: Genero) -> Cancion {
        Cancion { titulo, artista, genero }
    }
}

impl Playlist {

    fn new(nombre: String) -> Playlist {
        Playlist { nombre, canciones: Vec::new() }
    }

    // ➔ agregar canción.
    fn agregar_cancion(&mut self, cancion: Cancion) {
        self.canciones.push(cancion);
    }

    // ➔ eliminar canción.
    fn eliminar_cancion(&mut self, cancion: &Cancion) {
        self.canciones.retain_mut(|c| *c != *cancion);
    }

    // ➔ mover canción: mueve la canción a una determinada posición de la playlist.
    fn mover_cancion(&mut self, cancion: Cancion, posicion: usize) {
        if posicion > self.canciones.len()  { return; }
        self.eliminar_cancion(&cancion);
        self.canciones.insert(posicion - 1,  cancion);
    }

    // ➔ buscar canción por nombre.
    fn buscar_cancion(&self, nombre_cancion: &str) -> Option<&Cancion> {
        for cancion in &self.canciones {
            if cancion.titulo == nombre_cancion {
                return Some(cancion)
            }
        }
        None
    }

    // ➔ obtener las canciones de un determinado género.
    fn listar_canciones_genero(&self, genero: &Genero) -> Vec<&Cancion> {
        let mut vec: Vec<&Cancion> = Vec::new();

        for cancion in &self.canciones {
            if cancion.genero == *genero {
                vec.push(cancion);
            }
        }

        vec
    }

    // ➔ obtener las canciones de un determinado artista.
    fn listar_canciones_artista(&self, artista: &str) -> Vec<&Cancion> {
        let mut vec: Vec<&Cancion> = Vec::new();

        for cancion in &self.canciones {
            if cancion.artista == artista {
                vec.push(cancion);
            }
        }

        vec
    }

    // ➔ modificar título de la playlist.
    fn modificar_titulo(&mut self, titulo: String) {
        self.nombre = titulo;
    }

    // ➔ eliminar todas las canciones.
    fn clear(&mut self) {
        self.canciones.clear();
    }

}

fn main() {

}

#[cfg(test)]
mod tests {
    use std::cmp::PartialEq;
    use crate::{Cancion, Genero, Playlist};

    impl PartialEq<Cancion> for &Cancion {
        fn eq(&self, cancion: &Cancion) -> bool {
            cancion.titulo == self.titulo
            && cancion.artista == self.artista
            && cancion.genero == self.genero
        }
    }

    #[test]
    fn test() {
        let mut mi_playlist = Playlist::new("platuka".to_string());

        let mi_cancion_1 = Cancion::new("Un siglo sin tí".to_string(), "Chayanne".to_string(), Genero::Pop);
        let mi_cancion_2 = Cancion::new("Mi abuela".to_string(), "Molotov".to_string(), Genero::Rap);
        let mi_cancion_3 = Cancion::new("Te odio y te quiero".to_string(), "Julio Jaramillo".to_string(), Genero::Otros);
        let mi_cancion_4 = Cancion::new("Fatalidad".to_string(), "Julio Jaramillo".to_string(), Genero::Otros);

        mi_playlist.agregar_cancion(mi_cancion_1.clone());
        mi_playlist.agregar_cancion(mi_cancion_2.clone());
        mi_playlist.agregar_cancion(mi_cancion_3.clone());
        mi_playlist.agregar_cancion(mi_cancion_4.clone());

        // test agregado
        assert_eq!(mi_playlist.listar_canciones_genero(&Genero::Otros).len(), 2, "Deberían haber 2 canciones de género \"otros\"");

        // test mover
        mi_playlist.mover_cancion(mi_cancion_1.clone(), 4);
        assert_eq!(mi_playlist.canciones.get(3).unwrap(), mi_cancion_1, "La canción no parece haberse movido...");

        // test buscar genero
        assert_eq!(mi_playlist.listar_canciones_genero(&Genero::Otros).len(), 2, "Deberían haber 2 canciones en género Otros");

        // test clear
        mi_playlist.clear();
        assert_eq!(mi_playlist.canciones.len(), 0, "La playlist debería estar vacía");
    }

}