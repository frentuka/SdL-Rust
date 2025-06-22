use crate::structs::fecha::Fecha;

/// Para registrar un préstamo se requiere:
///     el libro,
///     el cliente,
///     la fecha de vencimiento del préstamo,
///     la fecha de devolución
///     y el estado (devuelto o en préstamo)
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Prestamo {
    pub isbn: u64, // isbn
    pub cliente: u32, // id
    pub vencimiento: Fecha,
    pub devolucion: Option<Fecha>,
    pub estado: EstadoPrestamo
}

#[derive(Default, Clone, PartialEq, PartialOrd, Debug)]
pub enum EstadoPrestamo {
    Devuelto, #[default] Prestando
}

impl Prestamo {
    
    pub fn new(isbn: u64, cliente: u32, vencimiento: Fecha, devolucion: Option<Fecha>, estado: EstadoPrestamo) -> Prestamo {
        Prestamo { isbn, cliente, vencimiento, devolucion, estado }
    }
    
}