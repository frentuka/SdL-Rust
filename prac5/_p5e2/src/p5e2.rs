use std::fs::File;
use std::io::Write;
use serde::Serialize;

//
// cancion.rs
//

#[derive(Serialize, PartialEq, Clone, Copy, Debug)]
pub enum Genero {
    Rock, Pop, Rap, Jazz, Otros
}

#[derive(Serialize, PartialEq, Clone, Debug)]
pub struct Cancion {
    pub titulo: String,
    pub artista: String,
    pub genero: Genero,
}

impl Cancion {
    pub fn new(titulo: &str, artista: &str, genero: Genero) -> Self {
        Self { titulo: titulo.to_string(), artista: artista.to_string(), genero }
    }
}

//
// playlist.rs
//

#[derive(Serialize)]
pub struct Playlist {
    pub nombre: String,
    pub canciones: Vec<Cancion>
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ErrorAgregarCancion {
    Archivos(ErrorSobreescribirArchivo)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ErrorMoverCancion {
    PosicionFueraDeLimites,
    CancionNoEncontrada,
    Archivos(ErrorSobreescribirArchivo)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ErrorEliminarCancion {
    CancionNoExiste,
    Archivos(ErrorSobreescribirArchivo)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ErrorSobreescribirArchivo {
    CrearArchivo,
    SerializarPlaylist,
    EscribirArchivo
}

impl Playlist {
    pub fn new(nombre: &str) -> Self {
        Self { nombre: nombre.to_string(), canciones: Vec::new() }
    }

    // ➔ agregar canción.
    pub fn agregar_cancion(&mut self, cancion: Cancion) -> Result<(), ErrorAgregarCancion> {
        self.canciones.push(cancion);

        match self.sobreescribir_archivo() {
            Ok(_) => { Ok(()) }
            Err(x) => { Err(ErrorAgregarCancion::Archivos(x)) }
        }
    }

    // ➔ eliminar canción.
    pub fn eliminar_cancion(&mut self, titulo: &str, artista: &str) -> Result<Cancion, ErrorEliminarCancion> {
        if let Some(index) = self.canciones.iter().position(|c| c.titulo == titulo && c.artista == artista) {
            let cancion = self.canciones.remove(index);
            match self.sobreescribir_archivo() {
                Ok(_) => { Ok(cancion) }
                Err(x) => { Err(ErrorEliminarCancion::Archivos(x)) }
            }
            // no usar swap_remove para mantener orden relativo
        } else {
            Err(ErrorEliminarCancion::CancionNoExiste)
        }
    }

    // ➔ mover canción: mueve la canción a una determinada posición de la playlist.
    pub fn mover_cancion(&mut self, titulo: &str, artista: &str, posicion: usize) -> Result<(), ErrorMoverCancion> {
        if posicion > self.canciones.len() || posicion < 1 {
            return Err(ErrorMoverCancion::PosicionFueraDeLimites);
        }

        let index_cancion = if let Some(index) = self.canciones.iter().position(|c| c.titulo == titulo && c.artista == artista) {
            index
        } else { return Err(ErrorMoverCancion::CancionNoEncontrada) };

        // mover
        let cancion = self.canciones.remove(index_cancion);
        self.canciones.insert(posicion - 1, cancion);

        match self.sobreescribir_archivo() {
            Ok(_) => { Ok(()) }
            Err(x) => { Err(ErrorMoverCancion::Archivos(x)) }
        }
    }

    // ➔ buscar canción por nombre
    pub fn buscar_cancion(&self, nombre_cancion: &str) -> Option<&Cancion> {
        self.canciones.iter().find(|cancion| cancion.titulo == nombre_cancion)
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
    pub fn modificar_titulo(&mut self, titulo: &str) -> Result<(), ErrorSobreescribirArchivo> {
        self.nombre = titulo.to_string();
        self.sobreescribir_archivo()
    }

    // ➔ eliminar todas las canciones.
    pub fn clear(&mut self) -> Result<(), ErrorSobreescribirArchivo> {
        self.canciones.clear();
        self.sobreescribir_archivo()
    }

    //      b- Una vez obtenido dicho coverage, las canciones de la playlist deben ser guardadas en un archivo en formato JSON,
    //          por lo tanto las operaciones que agreguen, quiten o modifiquen la playlist deben estar respaldadas sobre dicho archivo.

    fn sobreescribir_archivo(&self) -> Result<(), ErrorSobreescribirArchivo> {
        let mut file = match File::create(format!("{}.json", self.nombre)) {
            Ok(res) => { res }
            Err(_) => { return Err(ErrorSobreescribirArchivo::CrearArchivo) }
        };

        let json_data = match serde_json::to_string_pretty(self) {
            Ok(res) => { res }
            Err(_) => { return Err(ErrorSobreescribirArchivo::SerializarPlaylist) }
        };

        match file.write_all(json_data.as_bytes()) {
            Ok(_) => { Ok(()) }
            Err(_) => { Err(ErrorSobreescribirArchivo::EscribirArchivo) }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::PartialEq;
    use super::*;

    impl PartialEq<Cancion> for &Cancion {
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

        mi_playlist.agregar_cancion(mi_cancion_1.clone()).expect("TODO: panic message");
        mi_playlist.agregar_cancion(mi_cancion_2.clone()).expect("TODO: panic message");
        mi_playlist.agregar_cancion(mi_cancion_3.clone()).expect("TODO: panic message");
        mi_playlist.agregar_cancion(mi_cancion_4.clone()).expect("TODO: panic message");

        // test modificar titulo
        assert_eq!(mi_playlist.nombre, "platuka");
        mi_playlist.modificar_titulo("platukon");
        assert_eq!(mi_playlist.nombre, "platukon");

        // test agregado
        assert_eq!(mi_playlist.listar_canciones_genero(&Genero::Otros).len(), 2, "Deberían haber 2 canciones de género \"otros\"");

        // test mover
        mi_playlist.mover_cancion(mi_cancion_1.titulo.as_str(), mi_cancion_1.artista.as_str(), 4).expect("asdasdasdsa");
        assert_eq!(mi_playlist.canciones.get(3).unwrap(), mi_cancion_1, "La canción no parece haberse movido...");

        // test buscar genero
        assert_eq!(mi_playlist.listar_canciones_genero(&Genero::Otros).len(), 2, "Deberían haber 2 canciones en género Otros");

        // test listar canciones artista
        let listado = mi_playlist.listar_canciones_artista(mi_cancion_4.artista.as_str());
        assert_eq!(listado.len(), 2);
        assert!(listado.contains(&&mi_cancion_3));
        assert!(listado.contains(&&mi_cancion_4));

        // test clear
        mi_playlist.clear();
        assert_eq!(mi_playlist.canciones.len(), 0, "La playlist debería estar vacía");

        // test eliminar cancion
        mi_playlist.agregar_cancion(mi_cancion_1.clone());
        mi_playlist.agregar_cancion(mi_cancion_2.clone());

        let buscar = mi_playlist.buscar_cancion(mi_cancion_1.titulo.as_str());
        assert!(buscar.is_some());

        let cancion_eliminada = mi_playlist.eliminar_cancion(mi_cancion_1.titulo.as_str(), mi_cancion_1.artista.as_str()).expect("La canción debería haberse eliminado");
        assert_eq!(cancion_eliminada, mi_cancion_1, "La canción eliminada no es la esperada");

        let buscar = mi_playlist.buscar_cancion(mi_cancion_1.titulo.as_str());
        assert!(buscar.is_none());

        // test eliminar cancion: no existe
        let res = mi_playlist.eliminar_cancion(mi_cancion_3.titulo.as_str(), mi_cancion_3.artista.as_str());
        assert_eq!(res, Err(ErrorEliminarCancion::CancionNoExiste));

        // test mover cancion: pos. fuera de limites
        let res = mi_playlist.mover_cancion(mi_cancion_3.titulo.as_str(), mi_cancion_3.artista.as_str(), 13548);
        assert_eq!(res, Err(ErrorMoverCancion::PosicionFueraDeLimites));

        // test mover cancion: no existe
        let res = mi_playlist.mover_cancion(mi_cancion_3.titulo.as_str(), mi_cancion_3.artista.as_str(), 1);
        assert_eq!(res, Err(ErrorMoverCancion::CancionNoEncontrada));
        
        
    }
}