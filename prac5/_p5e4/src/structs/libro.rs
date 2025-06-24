use serde::{Deserialize, Deserializer, Serialize};

/// De cada libro se conoce:
///     el título,
///     autor,
///     número de páginas,
///     género (novela, infantil, técnico, otros).
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, Debug)]
pub(crate) struct Libro {
    pub isbn: u64,
    pub titulo: String,
    pub autor: String,
    pub paginas: u16,
    pub genero: Genero,
    pub stock: u32
}

// impl<'de: 'a, 'a> Deserialize<'de> for Libro<'a> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>
//     {
//         deserializer.deserialize_str();
//     }
// }

#[derive(Serialize, Deserialize, Default, Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum Genero {
    Novela, Infantil, Tecnico, #[default] Otros
}

impl Libro {
    pub fn new(isbn: u64, titulo: String, autor: String, paginas: u16, genero: Genero, stock: u32) -> Self {
        Self { isbn, titulo, autor, paginas, genero, stock }
    }
}