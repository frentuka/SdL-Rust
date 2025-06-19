use serde::Serialize;

#[derive(Serialize, PartialEq, Clone, Copy, Debug)]
pub enum Genero {
    Rock, Pop, Rap, Jazz, Otros
}

#[derive(Serialize, PartialEq, Clone, Debug)]
pub struct Cancion<'a> {
    pub titulo: &'a str,
    pub artista: &'a str,
    pub genero: Genero,
}

impl<'a> Cancion<'a> {
    pub fn new(titulo: &'a str, artista: &'a str, genero: Genero) -> Self {
        Self { titulo, artista, genero }
    }
}