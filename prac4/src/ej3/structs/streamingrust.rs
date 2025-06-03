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
use std::collections::{BTreeMap, HashMap, HashSet};
use crate::structs::suscripcion::TipoSuscripcion;
use crate::structs::usuario::Usuario;

// contienen la información necesaria para identificar la compra
//         mercadopago: cvu
//         credito: cbu, cuotas
//         debito: cbu
//         transferencia: cbu
//         cripto: public key/wallet address
//         combinación: set de cualquiera de las anteriores
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MedioDePago {
    Efectivo,
    MercadoPago(u128),
    Credito(u128, u8),
    Debito(u128),
    Transferencia(u128),
    Cripto([u8; 65])
}

impl MedioDePago {
    fn id(&self) -> u8 {
        match self {
            MedioDePago::Efectivo => 0,
            MedioDePago::MercadoPago(_) => 1,
            MedioDePago::Credito(_, _) => 2,
            MedioDePago::Debito(_) => 3,
            MedioDePago::Transferencia(_) => 4,
            MedioDePago::Cripto(_) => 5,
        }
    }

    fn from_id(id: u8) -> MedioDePago {
        match id {
            0 | 6..=u8::MAX => MedioDePago::Efectivo,
            1 => MedioDePago::MercadoPago(0),
            2 => MedioDePago::Credito(0, 0),
            3 => MedioDePago::Debito(0),
            4 => MedioDePago::Transferencia(0),
            5 => MedioDePago::Cripto([0; 65])
        }
    }
}

struct StreamingRust<'a> {
    usuarios: BTreeMap<u64, Usuario<'a>>
}

impl<'a> StreamingRust<'a> {
    
    fn new(usuarios: Option<BTreeMap<u64, Usuario>>) -> StreamingRust {
        StreamingRust { usuarios: usuarios.unwrap_or_default() }
    }

    fn contabilizar_medios_de_pago(&self, contabilizar_inactivos: bool) -> HashMap<MedioDePago, u32> {
        let mut contabilizador_mdp: HashMap<MedioDePago, u32> = HashMap::new();

        self.usuarios.iter().for_each(|(_, usuario)| {
            if contabilizar_inactivos || usuario.suscripcion.activo {
                if let Some(medio) = &usuario.suscripcion.medio_de_pago {
                    *contabilizador_mdp.entry(medio.clone()).or_insert(0)+= 1;
                }
            }
        });

        contabilizador_mdp
    }

    fn contabilizar_tipos_de_suscripcion(&self, contabilizar_inactivos: bool) -> HashMap<TipoSuscripcion, u32> {
        let mut contabilizador_ts: HashMap<TipoSuscripcion, u32> = HashMap::new();

        self.usuarios.iter().for_each(|(_, usuario)| {
            if contabilizar_inactivos || usuario.suscripcion.activo {
                *contabilizador_ts.entry(usuario.suscripcion.tipo_suscripcion.clone()).or_insert(0)+= 1;
            }
        });

        contabilizador_ts
    }

    // ➢ Saber el medio de pago que es más utilizado por los usuarios sobre las suscripciones activas
    fn medio_de_pago_mas_utilizado_activos(&self) -> Option<MedioDePago> {
        if self.usuarios.is_empty() { return None }

        let contabilizador_mdp = self.contabilizar_medios_de_pago(false);

        contabilizador_mdp.iter()
            .max_by_key(|(_, c)| *c)
            .map(|(mdp, _)| mdp.clone())
    }

    // ➢ Saber cuál fue el medio de pago más utilizado.
    fn medio_de_pago_mas_utilizado_general(&self) -> Option<MedioDePago> {
        if self.usuarios.is_empty() { return None }

        let contabilizador_mdp = self.contabilizar_medios_de_pago(true);

        contabilizador_mdp.iter()
            .max_by_key(|(_, c)| *c)
            .map(|(mdp, _)| mdp.clone())
    }

    // ➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones activas.
    fn suscripcion_activa_mas_contratada(&self) -> Option<TipoSuscripcion> {
        if self.usuarios.is_empty() { return None }

        let contabilizador_ts = self.contabilizar_tipos_de_suscripcion(false);

        contabilizador_ts.iter()
            .max_by_key(|(_, c)| *c)
            .map(|(ts, _)| ts.clone())
    }

    // ➢ Saber cuál fue la suscripción más contratada.
    fn suscripcion_mas_contratada_general(&self) -> Option<TipoSuscripcion> {
        if self.usuarios.is_empty() { return None }

        let contabilizador_ts = self.contabilizar_tipos_de_suscripcion(true);

        contabilizador_ts.iter()
            .max_by_key(|(_, c)| *c)
            .map(|(ts, _)| ts.clone())
    }
}