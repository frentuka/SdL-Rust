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
use std::collections::HashMap;
use error_proc_macro::Error;
use serde::{Deserialize, Serialize};
use crate::structs::fecha::Fecha;
use crate::structs::streamingrust::{ErrorDegradarSuscripcion, ErrorMejorarSuscripcion};

type HistorialSuscripciones = HashMap<u16, Suscripcion>;

// contienen la información necesaria para identificar la compra
//         mercadopago: cvu
//         credito: cbu, cuotas
//         debito: cbu
//         transferencia: cbu
//         cripto: public key/wallet address
//         combinación: set de cualquiera de las anteriores
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum MedioDePago {
    #[default]
    Efectivo,
    MercadoPago(u128),
    Credito(u128, u8),
    Debito(u128),
    Transferencia(u128),
    Cripto(String)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Precio(f64);
impl PartialEq for Precio {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_nan() || self.0.is_infinite() { return false }
        self.0 == other.0
    }
}
impl From<f64> for Precio {
    fn from(value: f64) -> Self { Precio(value) }
}
impl Precio {
    pub fn f64(&self) -> f64 { self.0 }
}

pub trait F64Precio {
    fn as_precio(self) -> Precio;
}
impl F64Precio for f64 {
    fn as_precio(self) -> Precio {
        Precio::from(self)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TipoSuscripcion {
    Basic, Classic, Super
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Suscripcion {
    pub id: u16, // (ID del usuario, número de suscripcion)
    pub tipo: TipoSuscripcion,
    pub costo_mensual: Precio,
    pub fecha_inicio: Fecha
}

impl TipoSuscripcion {
    pub fn costo_mensual(self) -> Precio {
        Precio::from(match self {
            TipoSuscripcion::Basic => 1200.0,
            TipoSuscripcion::Classic => 2200.0,
            TipoSuscripcion::Super => 5800.0
        })
    }
}

impl Suscripcion {
    pub fn new(id: u16, tipo: TipoSuscripcion, fecha_inicio: Fecha) -> Self {
        Suscripcion { id, tipo, costo_mensual: tipo.costo_mensual(), fecha_inicio }
    }

    pub fn mejorar(&mut self) -> Result<TipoSuscripcion, ErrorMejorarSuscripcion> {
        self.tipo = match &self.tipo {
            TipoSuscripcion::Basic => TipoSuscripcion::Classic,
            TipoSuscripcion::Classic => TipoSuscripcion::Super,
            TipoSuscripcion::Super => return Err(ErrorMejorarSuscripcion::SuscripcionMaxima)
        };

        self.costo_mensual = self.tipo.costo_mensual();
        Ok(self.tipo) // cheap copy. don't need to borrow
    }

    // reduce el rango de la suscripcion
    // opcionalmente puede cambiar el medio de pago y la fecha actual
    pub fn degradar(&mut self) -> Result<TipoSuscripcion, ErrorDegradarSuscripcion> {
        self.tipo = match &self.tipo {
            TipoSuscripcion::Basic => return Err(ErrorDegradarSuscripcion::SuscripcionMinima),
            TipoSuscripcion::Classic => TipoSuscripcion::Basic,
            TipoSuscripcion::Super => TipoSuscripcion::Classic
        };

        self.costo_mensual = self.tipo.costo_mensual();
        Ok(self.tipo)
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::fecha::Fecha;
    use crate::structs::suscripcion::{ErrorDegradarSuscripcion, ErrorMejorarSuscripcion, Precio, Suscripcion, TipoSuscripcion};

    #[test]
    fn test_new() {
        let tipo = TipoSuscripcion::Basic;
        let fecha = Fecha { dia: 1, mes: 1, ano: 1};

        let suscripcion_1 = Suscripcion {

            id: 0,
            tipo,
            costo_mensual: tipo.costo_mensual(),
            fecha_inicio: fecha,
        };

        let suscripcion_2 = Suscripcion::new(
            0,
            tipo,
            Fecha { dia: 1, mes: 1, ano: 1 }
        );

        assert_eq!(suscripcion_1, suscripcion_2);
    }

    #[test]
    fn test_precio_mejorar_degradar() {
        let mut suscripcion_basic = Suscripcion::new(
            0, TipoSuscripcion::Basic, Fecha { dia: 1, mes: 1, ano: 1}
        );

        let mut suscripcion_classic = Suscripcion::new(
            0, TipoSuscripcion::Classic, Fecha { dia: 1, mes: 1, ano: 1}
        );

        let mut suscripcion_super = Suscripcion::new(
            0, TipoSuscripcion::Super, Fecha { dia: 1, mes: 1, ano: 1}
        );

        // TipoSuscripcion::Basic => 1200.0,
        // TipoSuscripcion::Classic => 2200.0,
        // TipoSuscripcion::Super => 5800.0

        assert_eq!(suscripcion_basic.tipo.costo_mensual(), Precio::from(1200.0));
        assert_eq!(suscripcion_classic.tipo.costo_mensual(), Precio::from(2200.0));
        assert_eq!(suscripcion_super.tipo.costo_mensual(), Precio::from(5800.0));

        assert_eq!(suscripcion_basic.degradar(), Err(ErrorDegradarSuscripcion::SuscripcionMinima));
        assert_eq!(suscripcion_super.mejorar(), Err(ErrorMejorarSuscripcion::SuscripcionMaxima));

        assert_eq!(suscripcion_basic.mejorar(), Ok(TipoSuscripcion::Classic));
        assert_eq!(suscripcion_super.degradar(), Ok(TipoSuscripcion::Classic));
        assert_eq!(suscripcion_classic.mejorar(), Ok(TipoSuscripcion::Super));
        assert_eq!(suscripcion_classic.degradar(), Ok(TipoSuscripcion::Classic));
    }
}