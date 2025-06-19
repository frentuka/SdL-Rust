use std::fs::File;
use std::io::Write;
use error_proc_macro::Error;
use serde::Serialize;
use crate::structs::cancion::{Cancion, Genero};

#[derive(Serialize)]
pub struct Playlist<'a> {
    pub nombre: &'a str,
    pub canciones: Vec<Cancion<'a>>
}

pub enum ResultAgregarCancion {
    Exito(ResultSobreescribirArchivo),
    PlaylistLlena{ capacity: usize },
}

pub enum ResultMoverCancion {
    Exito(ResultSobreescribirArchivo),
    PosicionFueraDeLimites { limite: usize },
    CancionNoEncontrada
}

pub enum ResultEliminarCancion<'a> {
    Exito{ cancion: Cancion<'a>, resultado_sobreescribir_archivo: ResultSobreescribirArchivo },
    CancionNoExiste,
}

#[derive(Error)]
enum ResultSobreescribirArchivo {
    Exito,
    CrearArchivo,
    SerializarPlaylist,
    EscribirArchivo
}

impl<'a> Playlist<'a> {
    pub fn new(nombre: &'a str) -> Self {
        Self { nombre, canciones: Vec::new() }
    }

    // ➔ agregar canción.
    pub fn agregar_cancion(&mut self, cancion: Cancion<'a>) -> ResultAgregarCancion {
        if self.canciones.len() == self.canciones.capacity() { return ResultAgregarCancion::PlaylistLlena { capacity: self.canciones.len() } }
        self.canciones.push(cancion);
        ResultAgregarCancion::Exito(self.sobreescribir_archivo())
    }

    // ➔ eliminar canción.
    pub fn eliminar_cancion(&mut self, titulo: &'a str, artista: &'a str) -> ResultEliminarCancion {
        if let Some(index) = self.canciones.iter().position(|c| c.titulo == titulo && c.artista == artista) {
            let cancion = self.canciones.remove(index);
            ResultEliminarCancion::Exito { cancion, resultado_sobreescribir_archivo: self.sobreescribir_archivo() } // no usar swap_remove para mantener orden relativo
        } else {
            ResultEliminarCancion::CancionNoExiste
        }
    }

    // ➔ mover canción: mueve la canción a una determinada posición de la playlist.
    pub fn mover_cancion(&mut self, titulo: &'a str, artista: &'a str, posicion: usize) -> ResultMoverCancion {
        if posicion >= self.canciones.len() { return ResultMoverCancion::PosicionFueraDeLimites{ limite: self.canciones.len() - 1 } }

        let index_cancion = if let Some(index) = self.canciones.iter().position(|c| c.titulo == titulo && c.artista == artista) {
            index
        } else { return ResultMoverCancion::CancionNoEncontrada };

        // mover
        let cancion = self.canciones.remove(index_cancion);
        self.canciones.insert(posicion, cancion);

        ResultMoverCancion::Exito(self.sobreescribir_archivo())
    }

    // ➔ buscar canción por nombre. (no puede ser solo una :/)
    pub fn buscar_cancion(&self, nombre_cancion: &'a str) -> Vec<&Cancion> {
        let mut canciones_encontradas = Vec::with_capacity(self.canciones.capacity());
        for cancion in &self.canciones {
            if cancion.titulo == nombre_cancion {
                canciones_encontradas.push(cancion);
            }
        }
        canciones_encontradas
    }

    // ➔ obtener las canciones de un determinado género.
    pub fn listar_canciones_genero(&self, genero: &Genero) -> Vec<&Cancion> {
        let mut vec: Vec<&Cancion> = Vec::new();

        for cancion in &self.canciones {
            if cancion.genero == *genero {
                vec.push(cancion);
            }
        }

        vec
    }

    // ➔ obtener las canciones de un determinado artista.
    pub fn listar_canciones_artista(&self, artista: &str) -> Vec<&Cancion> {
        let mut vec: Vec<&Cancion> = Vec::new();

        for cancion in &self.canciones {
            if cancion.artista == artista {
                vec.push(cancion);
            }
        }

        vec
    }

    // ➔ modificar título de la playlist.
    pub fn modificar_titulo(&mut self, titulo: &'a str) -> ResultSobreescribirArchivo {
        self.nombre = titulo;
        self.sobreescribir_archivo()
    }

    // ➔ eliminar todas las canciones.
    pub fn clear(&mut self) -> ResultSobreescribirArchivo {
        self.canciones.clear();
        self.sobreescribir_archivo()
    }

    //      b- Una vez obtenido dicho coverage, las canciones de la playlist deben ser guardadas en un archivo en formato JSON,
    //          por lo tanto las operaciones que agreguen, quiten o modifiquen la playlist deben estar respaldadas sobre dicho archivo.

    fn sobreescribir_archivo(&self) -> ResultSobreescribirArchivo {
        let mut file = match File::create(format!("{}.json", self.nombre)) {
            Ok(res) => { res }
            Err(_) => { return ResultSobreescribirArchivo::CrearArchivo }
        };

        let json_data = match serde_json::to_string_pretty(self) {
            Ok(res) => { res }
            Err(_) => { return ResultSobreescribirArchivo::SerializarPlaylist }
        };

        match file.write(json_data.as_bytes()) {
            Ok(res) => { ResultSobreescribirArchivo::Exito },
            Err(_) => { ResultSobreescribirArchivo::EscribirArchivo }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::PartialEq;
    use crate::structs::cancion::{Cancion, Genero};
    use crate::structs::playlist::Playlist;

    impl<'a> PartialEq<Cancion<'a>> for &Cancion<'a> {
        fn eq(&self, cancion: &Cancion) -> bool {
            cancion.titulo == self.titulo
                && cancion.artista == self.artista
                && cancion.genero == self.genero
        }
    }

    #[test]
    fn test() {
        let mut mi_playlist = Playlist::new("platuka");

        let mi_cancion_1 = Cancion::new("Un siglo sin tí", "Chayanne", Genero::Pop);
        let mi_cancion_2 = Cancion::new("Mi abuela", "Molotov", Genero::Rap);
        let mi_cancion_3 = Cancion::new("Te odio y te quiero", "Julio Jaramillo", Genero::Otros);
        let mi_cancion_4 = Cancion::new("Fatalidad", "Julio Jaramillo", Genero::Otros);

        mi_playlist.agregar_cancion(mi_cancion_1.clone());
        mi_playlist.agregar_cancion(mi_cancion_2.clone());
        mi_playlist.agregar_cancion(mi_cancion_3.clone());
        mi_playlist.agregar_cancion(mi_cancion_4.clone());

        // test agregado
        assert_eq!(mi_playlist.listar_canciones_genero(&Genero::Otros).len(), 2, "Deberían haber 2 canciones de género \"otros\"");

        // test mover
        mi_playlist.mover_cancion(mi_cancion_1.titulo, mi_cancion_1.artista, 4);
        assert_eq!(mi_playlist.canciones.get(3).unwrap(), mi_cancion_1, "La canción no parece haberse movido...");

        // test buscar genero
        assert_eq!(mi_playlist.listar_canciones_genero(&Genero::Otros).len(), 2, "Deberían haber 2 canciones en género Otros");

        // test clear
        mi_playlist.clear();
        assert_eq!(mi_playlist.canciones.len(), 0, "La playlist debería estar vacía");
    }

}