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
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::structs::suscripcion::{MedioDePago, Suscripcion, TipoSuscripcion};

type HistorialSuscripciones = HashMap<u16, Suscripcion>;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Usuario {
    pub id: u64,
    pub email: String,
    pub medio_de_pago: MedioDePago,
    pub suscripcion_activa: Option<u16>,
    pub historial_suscripciones: HistorialSuscripciones,
}

impl Usuario {
    // ➢ Crear un usuario con una determinada suscripción y medio de pago.
    pub fn new(id: u64, email: String, medio_de_pago: MedioDePago, suscripcion_activa: Option<Suscripcion>) -> Self {
        // si se provee suscripción, generar historial con dicha suscripción
        let (historial, sus_id) = if let Some(suscripcion) = suscripcion_activa {
            (HashMap::from([(0u16, suscripcion)]), Some(0u16))
        } else { (HashMap::new(), None) };

        Usuario { id, email, medio_de_pago, suscripcion_activa: sus_id, historial_suscripciones: historial }
    }
    
    pub fn obtener_suscripcion(&mut self) -> Option<&mut Suscripcion> {
        let id_sus = self.suscripcion_activa?;
        
        match self.historial_suscripciones.get_mut(&id_sus) {
            None => { None }
            Some(sus) => { Some(sus) }
        }
    }

    // ➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic pasa a Clasic y si está en Clasic pasa a Super.
    // -> manejado en streamingrust.rs

    // ➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
    // -> manejado en streamingrust.rs

    // ➢ Dado un usuario cancelar la suscripción.
    pub fn cancelar_suscripcion(&mut self) -> bool {
        if self.suscripcion_activa.is_some() {
            self.suscripcion_activa = None;
            return true
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::fecha::Fecha;
    use crate::structs::suscripcion::{MedioDePago, Precio, Suscripcion, TipoSuscripcion};
    use crate::structs::usuario::Usuario;

    #[test]
    fn test_new() {
        let sus = Suscripcion {
            id: 0,
            tipo: TipoSuscripcion::Basic,
            costo_mensual: Precio::from(0.0),
            fecha_inicio: Fecha::default(),
        };
        
        let user1 = Usuario::new(0, "asd".to_string(), MedioDePago::Efectivo, Some(sus));
        let user2 = Usuario::new(0, "asd".to_string(), MedioDePago::Efectivo, None);
        
        assert_eq!(user1.historial_suscripciones.len(), 1);
        assert_eq!(user2.historial_suscripciones.len(), 0);
    }
    
    #[test]
    fn test_cancelar_suscripcion() {
        let sus = Suscripcion {
            id: 0,
            tipo: TipoSuscripcion::Basic,
            costo_mensual: Precio::from(0.0),
            fecha_inicio: Fecha::default(),
        };

        let mut user1 = Usuario::new(0, "asd".to_string(), MedioDePago::Efectivo, Some(sus));
        let mut user2 = Usuario::new(0, "asd".to_string(), MedioDePago::Efectivo, None);

        assert!(user1.obtener_suscripcion().is_some(), "Tiene suscripción");
        assert!(user2.obtener_suscripcion().is_none(), "No tiene suscripción");

        assert!(user1.cancelar_suscripcion());
        assert!(!user2.cancelar_suscripcion());
    }
}