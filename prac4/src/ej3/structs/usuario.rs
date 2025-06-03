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
use crate::structs::errores::{ErrorDegradarSuscripcion, ErrorMejorarSuscripcion};
use crate::structs::suscripcion::{Suscripcion, TipoSuscripcion};

pub struct Usuario<'a> {
    pub id: u64,
    pub email: &'a str,
    pub suscripcion: Suscripcion,
}

impl<'a> Usuario<'a> {
    // ➢ Crear un usuario con una determinada suscripción y medio de pago.
    fn new(id: u64, email: &'a str, suscripcion: Suscripcion) -> Self {
        Usuario { id, email, suscripcion }
    }

    // ➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic pasa a Clasic y si está en Clasic pasa a Super.
    fn mejorar_suscripcion(&mut self) -> Result<&TipoSuscripcion, ErrorMejorarSuscripcion> {
        self.suscripcion.mejorar()
    }

    // ➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
    fn degradar_suscripcion(&mut self) -> Result<&TipoSuscripcion, ErrorDegradarSuscripcion> {
        self.suscripcion.degradar()
    }

    // ➢ Dado un usuario cancelar la suscripción.
    fn cancelar_suscripcion(&mut self) -> bool {
        let estaba_activa = self.suscripcion.activo;
        self.suscripcion.activo = false;
        estaba_activa
    }

}