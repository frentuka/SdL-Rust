/*

3 -La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones (Basic, Clasic, Super) a sus usuarios.
    Cada suscripción tiene un costo mensual, una duración en meses y una fecha de inicio.
    Además, los usuarios pueden pagar por sus suscripciones con distintos medios de pago
        que son Efectivo, MercadoPago, Tarjeta de Crédito, Transferencia Bancaria, o Cripto.
    Cada medio de pago tiene sus datos correspondientes a excepción de Efectivo.
    Los usuarios solo pueden tener una suscripción activa a la vez.
    Implemente las estructuras, funciones asociadas y traits necesarios para resolver las siguientes acciones:

➢ Crear un usuario con una determinada suscripción y medio de pago.
➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic pasa a Clasic y si está en Clasic pasa a Super.
➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
➢ Dado un usuario cancelar la suscripción.
➢ Saber el medio de pago que es más utilizado por los usuarios sobre las suscripciones activas
➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones activas.
➢ Saber cuál fue el medio de pago más utilizado.
➢ Saber cuál fue la suscripción más contratada.

*/
use std::cmp::PartialEq;
use crate::structs::errores::{ErrorDegradarSuscripcion, ErrorMejorarSuscripcion, ErrorNewSuscripcion};
use crate::structs::fecha::Fecha;
use crate::structs::streamingrust::MedioDePago;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TipoSuscripcion {
    Basic, Classic, Super
}

pub struct Suscripcion {
    pub activo: bool,
    pub tipo_suscripcion: TipoSuscripcion,
    pub medio_de_pago: Option<MedioDePago>,
    pub costo_mensual: f64,
    pub fecha_inicio: Option<Fecha>
}

impl From<ErrorNewSuscripcion> for ErrorMejorarSuscripcion {
    fn from(value: ErrorNewSuscripcion) -> Self {
        match value {
            ErrorNewSuscripcion::FechaInvalida => ErrorMejorarSuscripcion::FechaInvalida,
            ErrorNewSuscripcion::MedioDePagoInvalido => ErrorMejorarSuscripcion::MedioDePagoInvalido
        }
    }
}

impl From<ErrorNewSuscripcion> for ErrorDegradarSuscripcion {
    fn from(value: ErrorNewSuscripcion) -> Self {
        match value {
            ErrorNewSuscripcion::FechaInvalida => ErrorDegradarSuscripcion::FechaInvalida,
            ErrorNewSuscripcion::MedioDePagoInvalido => ErrorDegradarSuscripcion::MedioDePagoInvalido
        }
    }
}

impl TipoSuscripcion {
    fn costo_mensual(&self) -> f64 {
        match self {
            TipoSuscripcion::Basic => 1200.0,
            TipoSuscripcion::Classic => 2200.0,
            TipoSuscripcion::Super => 5800.0
        }
    }
}

impl Suscripcion {

    pub(crate) fn new(tipo_suscripcion: TipoSuscripcion, activo: bool, medio_de_pago: Option<MedioDePago>, fecha_inicio: Option<Fecha>) -> Result<Self, ErrorNewSuscripcion> {
        if activo {
            if let None = medio_de_pago { return Err(ErrorNewSuscripcion::MedioDePagoInvalido) }
            if let None = fecha_inicio { return Err(ErrorNewSuscripcion::FechaInvalida) }
            if let Some(fecha) = &fecha_inicio { if !fecha.es_fecha_valida() { return Err(ErrorNewSuscripcion::FechaInvalida) } }
        }

        // el costo mensual no debería darse como argumento de la función
        // sino tener un costo predefinido para cada tipo de suscripción
        let costo_mensual = tipo_suscripcion.costo_mensual();

        Ok(Suscripcion { tipo_suscripcion, activo, medio_de_pago, costo_mensual, fecha_inicio })
    }

    pub fn mejorar(&mut self) -> Result<&TipoSuscripcion, ErrorMejorarSuscripcion> {
        let sig_suscripcion = match &self.tipo_suscripcion {
            TipoSuscripcion::Basic => TipoSuscripcion::Classic,
            TipoSuscripcion::Classic => TipoSuscripcion::Super,
            TipoSuscripcion::Super => return Err(ErrorMejorarSuscripcion::SuscripcionMaxima)
        };

        self.costo_mensual = sig_suscripcion.costo_mensual();
        self.tipo_suscripcion = sig_suscripcion;

        Ok(&self.tipo_suscripcion)
    }

    // reduce el rango de la suscripcion
    // opcionalmente puede cambiar el medio de pago y la fecha actual
    pub fn degradar(&mut self) -> Result<&TipoSuscripcion, ErrorDegradarSuscripcion> {
        let prev_suscripcion = match &self.tipo_suscripcion {
            TipoSuscripcion::Basic => return Err(ErrorDegradarSuscripcion::SuscripcionMinima),
            TipoSuscripcion::Classic => TipoSuscripcion::Basic,
            TipoSuscripcion::Super => TipoSuscripcion::Classic
        };

        self.costo_mensual = prev_suscripcion.costo_mensual();
        self.tipo_suscripcion = prev_suscripcion;

        Ok(&self.tipo_suscripcion)
    }
}