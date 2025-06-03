use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ErrorNewSuscripcion {
    FechaInvalida, MedioDePagoInvalido
}

#[derive(Debug)]
pub enum ErrorMejorarSuscripcion {
    SuscripcionMaxima, MedioDePagoInvalido, FechaInvalida
}

#[derive(Debug)]
pub enum ErrorDegradarSuscripcion {
    SuscripcionMinima, MedioDePagoInvalido, FechaInvalida,
}

impl fmt::Display for ErrorNewSuscripcion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorNewSuscripcion::FechaInvalida => write!(f, "La fecha ingresada es inválida"),
            ErrorNewSuscripcion::MedioDePagoInvalido => write!(f, "El medio de pago ingresado es inválido")
        }
    }
}

impl fmt::Display for ErrorMejorarSuscripcion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorMejorarSuscripcion::SuscripcionMaxima => write!(f, "La suscripción no puede mejorarse porque ya es la más alta posible"),
            ErrorMejorarSuscripcion::MedioDePagoInvalido => write!(f, "El medio de pago ingresado es inválido"),
            ErrorMejorarSuscripcion::FechaInvalida => write!(f, "La fecha ingresada es inválida")
        }
    }
}

impl fmt::Display for ErrorDegradarSuscripcion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorDegradarSuscripcion::SuscripcionMinima => write!(f, "La suscripción no puede mejorarse porque ya es la más alta posible"),
            ErrorDegradarSuscripcion::MedioDePagoInvalido => write!(f, "El medio de pago ingresado es inválido"),
            ErrorDegradarSuscripcion::FechaInvalida => write!(f, "La fecha ingresada es inválida")
        }
    }
}

impl Error for ErrorNewSuscripcion { }
impl Error for ErrorMejorarSuscripcion { }
impl Error for ErrorDegradarSuscripcion { }