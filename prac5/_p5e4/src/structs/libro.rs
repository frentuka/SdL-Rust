use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, Debug)]
pub enum Genero {
    Novela, Infantil, Tecnico, #[default] Otros
}

impl Libro {
    pub fn new(isbn: u64, titulo: String, autor: String, paginas: u16, genero: Genero, stock: u32) -> Self {
        Self { isbn, titulo, autor, paginas, genero, stock }
    }
}