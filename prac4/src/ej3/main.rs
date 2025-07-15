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
// mod structs;

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{BTreeMap, HashMap};
use std::fmt;

//
// fecha.rs
//

const NOMBRE_MESES: [&str; 12] = ["Enero", "Febrero", "Marzo", "Abril",
    "Mayo", "Junio", "Julio", "Agosto",
    "Septiembre", "Octubre", "Noviembre", "Diciembre"];
#[derive(Clone, PartialEq, Debug)]
pub struct Fecha {
    pub(crate) dia: u8,
    pub(crate) mes: u8,
    pub(crate) ano: i64
}

impl Default for Fecha {
    fn default() -> Self {
        Fecha { dia: 1, mes: 1, ano: 0 }
    }
}

impl PartialOrd for Fecha {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.dia == other.dia
            && self.mes == other.mes
            && self.ano == other.ano
        { return Some(Equal) }

        if self.ano > other.ano { return Some(Greater) }
        if self.mes > other.mes { return Some(Greater) }
        if self.dia > other.dia { return Some(Greater) }

        Some(Less)
    }
}

impl fmt::Display for Fecha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.es_fecha_valida() {
            write!(f, "{} de {} del {}", self.dia, NOMBRE_MESES[self.mes as usize - 1], self.ano)
        } else {
            write!(f, "{}/{}/{}", self.dia, self.mes, self.ano)
        }
    }
}

impl Fecha {

    // El año podría ser negativo, indicando días antes de Cristo.
    pub fn new(dia: u8, mes: u8, ano: i64) -> Option<Fecha> {
        let fecha = Fecha { dia, mes, ano };
        if fecha.es_fecha_valida() {
            return Some(fecha);
        }
        None
    }

    pub fn es_fecha_valida(&self) -> bool {
        // check que el mes sea válido
        if !(1..=12).contains(&self.mes) { return false }

        // check días del mes
        if self.dia == 0
            || self.dia > self.dias_mes_actual()
        { return false }

        // el año no puede ser incorrecto...
        // a no ser que se contabilice la edad del universo
        // que dudo mucho que pueda importar para este caso
        true
    }

    pub fn es_bisiesto(&self) -> bool {
        self.ano % 4 == 0
    }

    pub fn sumar_dias(&mut self, dias: u32) {
        let mut dias_restantes = dias;

        while dias_restantes > 0 {
            let dias_mes_actual = self.dias_mes_actual();
            let dias_para_proximo_mes = (dias_mes_actual - self.dia + 1) as u32;

            if dias_restantes >= dias_para_proximo_mes {
                // ir al siguiente mes

                dias_restantes-= dias_para_proximo_mes;
                self.dia = 1;
                self.mes += 1;

                if self.mes > 12 {
                    self.mes = 1;
                    self.ano+= 1;
                }
            } else {
                self.dia+= dias_restantes as u8;
                dias_restantes = 0;
            }
        }
    }

    pub fn restar_dias(&mut self, dias: u32) {
        let mut dias_restantes = dias;

        while dias_restantes > 0 {
            if dias_restantes >= self.dia as u32 {
                // ir al anterior mes
                dias_restantes-= self.dia as u32;
                self.mes-= 1;

                if self.mes < 1 {
                    self.mes = 12;
                    self.ano-= 1;
                }

                // corregir self.dia == 0
                self.dia = self.dias_mes_actual();
            } else {
                self.dia-= dias_restantes as u8;
                dias_restantes = 0;
            }
        }
    }

    pub fn dias_mes_actual(&self) -> u8 {
        match self.mes {
            4 | 6 | 9 | 11 => 30,
            2 => if self.es_bisiesto() { 29 } else { 28 },
            _ => 31,
        }
    }
}

//
// errores.rs
//

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

//
// suscripcion.rs
//

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
    pub(crate) fn new(tipo_suscripcion: TipoSuscripcion, activo: bool, medio_de_pago: Option<&MedioDePago>, fecha_inicio: Option<Fecha>) -> Result<Self, ErrorNewSuscripcion> {
        if activo {
            if let None = medio_de_pago { return Err(ErrorNewSuscripcion::MedioDePagoInvalido) }
            if let None = fecha_inicio { return Err(ErrorNewSuscripcion::FechaInvalida) }
            if let Some(fecha) = &fecha_inicio { if !fecha.es_fecha_valida() { return Err(ErrorNewSuscripcion::FechaInvalida) } }
        }

        // el costo mensual no debería darse como argumento de la función
        // sino tener un costo predefinido para cada tipo de suscripción
        let costo_mensual = tipo_suscripcion.costo_mensual();

        Ok(Suscripcion { tipo_suscripcion, activo, medio_de_pago: medio_de_pago.cloned(), costo_mensual, fecha_inicio })
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

//
// usuario.rs
//

pub struct Usuario {
    pub id: u64,
    pub email: String,
    pub suscripcion: Suscripcion,
}

impl Usuario {
    // ➢ Crear un usuario con una determinada suscripción y medio de pago.
    fn new(id: u64, email: &str, suscripcion: Suscripcion) -> Self {
        Usuario {
            id,
            email: email.to_string(),
            suscripcion
        }
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

//
// streamingrust.rs
//

struct StreamingRust {
    usuarios: BTreeMap<u64, Usuario>
}

impl StreamingRust {

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

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fecha() {
        let fecha = Fecha::new(29, 2, 2020).unwrap();
        assert!(fecha.es_bisiesto());
        assert_eq!(fecha.dias_mes_actual(), 29);
        assert_eq!(fecha.to_string(), "29 de Febrero del 2020");

        let mut fecha2 = Fecha::new(31, 12, 2021).unwrap();
        fecha2.sumar_dias(1);
        assert_eq!(fecha2.to_string(), "1 de Enero del 2022");

        fecha2.restar_dias(1);
        assert_eq!(fecha2.to_string(), "31 de Diciembre del 2021");
    }

    #[test]
    fn test_suscripcion() {
        let fecha_inicio = Fecha::new(1, 1, 2023).unwrap();
        let medio_pago = Some(MedioDePago::MercadoPago(123456789));
        let suscripcion = Suscripcion::new(TipoSuscripcion::Basic, true, medio_pago.as_ref(), Some(fecha_inicio)).unwrap();

        assert_eq!(suscripcion.tipo_suscripcion, TipoSuscripcion::Basic);
        assert!(suscripcion.activo);
        assert_eq!(suscripcion.costo_mensual, 1200.0);

        let mut usuario = Usuario::new(1, "asd@asd.asd", suscripcion);
        assert_eq!(usuario.suscripcion.tipo_suscripcion, TipoSuscripcion::Basic);
        assert!(usuario.suscripcion.activo);
        assert_eq!(usuario.suscripcion.costo_mensual, 1200.0);

        let resultado_mejora = usuario.mejorar_suscripcion();
        assert!(resultado_mejora.is_ok());
        assert_eq!(usuario.suscripcion.tipo_suscripcion, TipoSuscripcion::Classic);

        let resultado_degradacion = usuario.degradar_suscripcion();
        assert!(resultado_degradacion.is_ok());
        assert_eq!(usuario.suscripcion.tipo_suscripcion, TipoSuscripcion::Basic);

        let resultado_cancelacion = usuario.cancelar_suscripcion();
        assert!(resultado_cancelacion);
        assert!(!usuario.suscripcion.activo);
    }

    #[test]
    fn test_streamingrust() {
        let fecha_inicio = Fecha::new(1, 1, 2023).unwrap();
        let medio_pago = Some(MedioDePago::MercadoPago(123456789));
        let medio_pago2 = Some(MedioDePago::Efectivo);
        let suscripcion1 = Suscripcion::new(TipoSuscripcion::Basic, true, medio_pago.as_ref(), Some(fecha_inicio.clone())).unwrap();
        let suscripcion2 = Suscripcion::new(TipoSuscripcion::Classic, false, medio_pago2.as_ref(), Some(fecha_inicio.clone())).unwrap();
        let suscripcion3 = Suscripcion::new(TipoSuscripcion::Classic, false, medio_pago2.as_ref(), Some(fecha_inicio.clone())).unwrap();

        let usuario1 = Usuario::new(1, "asd@asd.asd", suscripcion1);
        let usuario2 = Usuario::new(2, "asd2@asd.asd", suscripcion2);
        let usuario3 = Usuario::new(3, "asd2@asd.asd", suscripcion3);

        let mut usuarios: BTreeMap<u64, Usuario> = BTreeMap::new();
        usuarios.insert(usuario1.id, usuario1);
        usuarios.insert(usuario2.id, usuario2);
        usuarios.insert(usuario3.id, usuario3);

        let streaming = StreamingRust::new(Some(usuarios));

        let medio_pago_mas_utilizado_activos = streaming.medio_de_pago_mas_utilizado_activos();
        assert!(medio_pago_mas_utilizado_activos.is_some());
        assert!(matches!(medio_pago_mas_utilizado_activos.unwrap(), MedioDePago::MercadoPago(_)));

        let medio_pago_mas_utilizado_general = streaming.medio_de_pago_mas_utilizado_general();
        assert!(medio_pago_mas_utilizado_general.is_some());
        assert!(matches!(medio_pago_mas_utilizado_general.unwrap(), MedioDePago::Efectivo));

        let suscripcion_activa_mas_contratada = streaming.suscripcion_activa_mas_contratada();
        assert!(suscripcion_activa_mas_contratada.is_some());
        assert_eq!(suscripcion_activa_mas_contratada.unwrap(), TipoSuscripcion::Basic);

        let suscripcion_mas_contratada_general = streaming.suscripcion_mas_contratada_general();
        assert!(suscripcion_mas_contratada_general.is_some());
        assert_eq!(suscripcion_mas_contratada_general.unwrap(), TipoSuscripcion::Classic);

        let contabilizador_mdp = streaming.contabilizar_medios_de_pago(false);
        assert_eq!(contabilizador_mdp.len(), 1);

        let contabilizador_ts = streaming.contabilizar_tipos_de_suscripcion(false);
        assert_eq!(contabilizador_ts.len(), 1);

        let contabilizador_mdp = streaming.contabilizar_medios_de_pago(true);
        assert_eq!(contabilizador_mdp.len(), 2);

        let contabilizador_ts = streaming.contabilizar_tipos_de_suscripcion(true);
        assert_eq!(contabilizador_ts.len(), 2);
    }

    #[test]
    fn test_fecha_invalida() {
        let fecha_invalida = Fecha::new(31, 2, 2021);
        assert!(fecha_invalida.is_none());

        let fecha_invalida = Fecha::new(29, 2, 2021);
        assert!(fecha_invalida.is_none());

        let fecha_invalida = Fecha::new(32, 1, 2021);
        assert!(fecha_invalida.is_none());

        let fecha_invalida = Fecha::new(0, 1, 2021);
        assert!(fecha_invalida.is_none());
    }

    #[test]
    fn test_fecha_valida() {
        let fecha_valida = Fecha::new(29, 2, 2020);
        assert!(fecha_valida.is_some());

        let fecha_valida = Fecha::new(31, 12, 2021);
        assert!(fecha_valida.is_some());

        let fecha_valida = Fecha::new(1, 1, 2021);
        assert!(fecha_valida.is_some());
    }

    #[test]
    fn test_suscripcion_invalida() {
        let fecha_inicio = Fecha::new(1, 1, 2023).unwrap();
        let medio_pago = Some(MedioDePago::MercadoPago(123456789));
        let suscripcion_invalida = Suscripcion::new(TipoSuscripcion::Basic, true, None, Some(fecha_inicio));
        assert!(suscripcion_invalida.is_err());

        let suscripcion_invalida = Suscripcion::new(TipoSuscripcion::Basic, true, medio_pago.as_ref(), None);
        assert!(suscripcion_invalida.is_err());

        let fecha_invalida = Fecha::new(31, 2, 2021);
        let suscripcion_invalida = Suscripcion::new(TipoSuscripcion::Basic, true, medio_pago.as_ref(), fecha_invalida);
        assert!(suscripcion_invalida.is_err());
    }
    
    #[test]
    fn test_suscripcion_valida() {
        let fecha_inicio = Fecha::new(1, 1, 2023).unwrap();
        let medio_pago = Some(MedioDePago::MercadoPago(123456789));
        let suscripcion_valida = Suscripcion::new(TipoSuscripcion::Basic, true, medio_pago.as_ref(), Some(fecha_inicio));
        assert!(suscripcion_valida.is_ok());
    }
}