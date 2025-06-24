use serde::{Deserialize, Serialize};
use crate::structs::fecha::Fecha;

/// Para registrar un préstamo se requiere:
///     el libro,
///     el cliente,
///     la fecha de vencimiento del préstamo,
///     la fecha de devolución
///     y el estado (devuelto o en préstamo)
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub struct Prestamo {
    pub isbn: u64, // isbn
    pub cliente: u32, // id
    pub vencimiento: Fecha,
    pub estado: EstadoPrestamo
}

#[derive(Serialize, Deserialize, Default, Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum EstadoPrestamo {
    Devuelto(Fecha), #[default] Prestando
}

impl Prestamo {
    
    pub fn new(isbn: u64, cliente: u32, vencimiento: Fecha, estado: EstadoPrestamo) -> Prestamo {
        Prestamo { isbn, cliente, vencimiento, estado }
    }
    
}