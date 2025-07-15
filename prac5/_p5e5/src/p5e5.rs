


//
// fecha.rs
//

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{BTreeMap, HashMap};
use std::{fmt, fs};
use std::fs::File;
use std::io::Read;
use std::mem::{discriminant, Discriminant};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;

const NOMBRE_MESES: [&str; 12] = ["Enero", "Febrero", "Marzo", "Abril",
    "Mayo", "Junio", "Julio", "Agosto",
    "Septiembre", "Octubre", "Noviembre", "Diciembre"];
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
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
            let dias_para_proximo_mes = u32::from(dias_mes_actual - self.dia + 1);

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
                self.dia += u8::try_from(dias_restantes).unwrap_or(0);
                dias_restantes = 0;
            }
        }
    }

    pub fn restar_dias(&mut self, dias: u32) {
        let mut dias_restantes = dias;

        while dias_restantes > 0 {
            if dias_restantes >= u32::from(self.dia) {
                // ir al anterior mes
                dias_restantes-= u32::from(self.dia);
                self.mes-= 1;

                if self.mes < 1 {
                    self.mes = 12;
                    self.ano-= 1;
                }

                // corregir self.dia == 0
                self.dia = self.dias_mes_actual();
            } else {
                self.dia-= u8::try_from(dias_restantes).unwrap_or(0);
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

#[cfg(test)]
mod test_fecha {
    use super::*;

    #[test]
    fn test_default() {
        // default: 01/01/0000
        let default_fecha = Fecha::default();
        assert_eq!(default_fecha.dia, 1);
        assert_eq!(default_fecha.mes, 1);
        assert_eq!(default_fecha.ano, 0);
    }

    #[test]
    fn test_display() {
        let valid_fecha = Fecha::default();
        let invalid_fecha = Fecha { dia: 0, mes: 1, ano: 0 };

        // write!(f, "{} de {} del {}", self.dia, NOMBRE_MESES[self.mes as usize - 1], self.ano)

        assert_ne!(format!("{valid_fecha}"), format!("{}", invalid_fecha));
        assert_eq!(format!("{valid_fecha}"), format!("{} de {} del {}", valid_fecha.dia, NOMBRE_MESES[valid_fecha.mes as usize - 1], valid_fecha.ano));
        assert_eq!(format!("{invalid_fecha}"), format!("{}/{}/{}", invalid_fecha.dia, invalid_fecha.mes, invalid_fecha.ano));
    }

    #[test]
    fn test_new() {
        // invalida
        let fecha = Fecha::new(0, 0, 0);
        assert!(fecha.is_none());

        // valida
        let fecha = Fecha::new(22, 08, 2002);
        assert!(fecha.is_some());
    }

    #[test]
    fn test_bisiesto() {
        let Some(fecha) = Fecha::new(1, 1, 0) else { panic!() };
        assert!(fecha.es_bisiesto());

        let Some(fecha) = Fecha::new(1, 1, 2000) else { panic!() };
        assert!(fecha.es_bisiesto());

        let Some(fecha) = Fecha::new(1, 1, -4) else { panic!() };
        assert!(fecha.es_bisiesto());

        let Some(fecha) = Fecha::new(1, 1, 1) else { panic!() };
        assert!(!fecha.es_bisiesto());
    }

    #[test]
    fn test_restar_dias() {
        let Some(mut fecha) = Fecha::new(30, 04, 2016) else { panic!() };

        fecha.restar_dias(5000);

        assert_eq!(fecha.dia, 22);
        assert_eq!(fecha.mes, 08);
        assert_eq!(fecha.ano, 2002);
    }

    #[test]
    fn test_sumar_dias() {
        let Some(mut fecha) = Fecha::new(22, 08, 2002) else { panic!() };

        fecha.sumar_dias(5000);

        assert_eq!(fecha.dia, 30);
        assert_eq!(fecha.mes, 04);
        assert_eq!(fecha.ano, 2016);
    }

    #[test]
    fn test_dias_mes_actual() {
        let Some(fecha) = Fecha::new(22, 01, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 31);
        let Some(fecha) = Fecha::new(22, 02, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 28);
        let Some(fecha) = Fecha::new(22, 02, 2004) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 29);
        let Some(fecha) = Fecha::new(22, 03, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 31);
        let Some(fecha) = Fecha::new(22, 04, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 30);
        let Some(fecha) = Fecha::new(22, 05, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 31);
        let Some(fecha) = Fecha::new(22, 06, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 30);
        let Some(fecha) = Fecha::new(22, 07, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 31);
        let Some(fecha) = Fecha::new(22, 08, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 31);
        let Some(fecha) = Fecha::new(22, 09, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 30);
        let Some(fecha) = Fecha::new(22, 10, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 31);
        let Some(fecha) = Fecha::new(22, 11, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 30);
        let Some(fecha) = Fecha::new(22, 12, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 31);
    }

    #[test]
    fn test_cmp() {
        let fecha1 = Fecha { dia: 1, mes: 1, ano: 1};
        let fecha2 = Fecha { dia: 3, mes: 1, ano: 1};
        let fecha3 = Fecha { dia: 3, mes: 1, ano: 1};

        assert!(fecha1 < fecha2, "Fecha 1 es anterior, por ende, es menor");
        assert_eq!(fecha3, fecha2, "Fecha 3 es igual a fecha 2");
        assert!(fecha3 > fecha1, "Fecha 3 es posterior a fecha1, por ende, es mayor");
    }
}

//
// suscripcion.rs
//

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
mod tests_suscripcion {
    use super::*;

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

//
// usuario.rs
//

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
mod test_usuario {
    use super::*;

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

//
// streamingrust_fm.rs
//

const BASE_FOLDER: &str = "";
const DEFAULT_FILE_NAME: &str = "streamingrust_usuarios";

pub trait UsuariosFile {
    fn unwrap_or_leer(self, file_name: &str) -> Usuarios;
}

impl UsuariosFile for Option<Usuarios> {
    fn unwrap_or_leer(self, file_name: &str) -> Usuarios {
        if let Some(usuarios) = self {
            usuarios
        } else {
            let Ok(value) = leer_archivo(format!("{BASE_FOLDER}{file_name}"))
            else { return Usuarios::new(); };

            leer_archivo_parsed::<Usuarios>(value).unwrap_or_default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum ErrorSobreescribirArchivo {
    IOError, //io::Error == ()
    SerializationError,
}

#[derive(Debug, Clone)]
pub enum ErrorLeerArchivo {
    IOError,
    DeserializationError,
}

impl PartialEq for ErrorSobreescribirArchivo {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == std::mem::discriminant(other)
    }
}

impl PartialEq for ErrorLeerArchivo {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == std::mem::discriminant(other)
    }
}

fn archivo_filepath(file_name: &str) -> String {
    format!("{BASE_FOLDER}{file_name}.json")
}

//
// IO part
//

fn sobreescribir_archivo(file_abs_path: String, data: &Usuarios) -> Result<(), ErrorSobreescribirArchivo> {
    let json_string = serde_json::to_string_pretty(data);

    let Ok(text) = json_string else { return Err(ErrorSobreescribirArchivo::SerializationError); };

    match fs::write(file_abs_path, text) {
        Err(_) => Err(ErrorSobreescribirArchivo::IOError),
        _ => Ok(())
    }
}

fn leer_archivo(filepath: String) -> Result<Value, ErrorLeerArchivo> {
    let Ok(mut file) = File::open(filepath) else { return Err(ErrorLeerArchivo::IOError) };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(_) => return Err(ErrorLeerArchivo::IOError)
    }

    match serde_json::from_str(&contents) {
        Err(_) => Err(ErrorLeerArchivo::DeserializationError),
        Ok(value) => Ok(value),
    }
}

fn leer_archivo_parsed<T>(value: Value) -> Result<T, ErrorLeerArchivo> where T: DeserializeOwned {
    let data: T = match serde_json::from_value::<T>(value) {
        Ok(data) => { data }
        Err(_) => { return Err(ErrorLeerArchivo::DeserializationError) }
    };
    Ok(data)
}

//
// Implementation / Serialization-Deserialization part
//

pub trait StreamingRustFileManagement {
    fn sobreescribir_archivo_usuarios(&self, file_name: &str) -> Result<(), ErrorSobreescribirArchivo>;
    fn leer_archivo_usuarios(&self) -> Result<Usuarios, ErrorLeerArchivo>;
    fn actualizar_variable_usuarios(&mut self) -> Result<(), ErrorLeerArchivo>;
}

impl StreamingRustFileManagement for StreamingRust {
    fn sobreescribir_archivo_usuarios(&self, file_name: &str) -> Result<(), ErrorSobreescribirArchivo> {
        let data = &self.usuarios;
        sobreescribir_archivo(archivo_filepath(file_name), data)
    }

    fn leer_archivo_usuarios(&self) -> Result<Usuarios, ErrorLeerArchivo> {
        let lectura = leer_archivo(archivo_filepath(self.file_name.as_str()))?;
        let usuarios: Usuarios = leer_archivo_parsed(lectura)?;
        Ok(usuarios)
    }

    fn actualizar_variable_usuarios(&mut self) -> Result<(), ErrorLeerArchivo> {
        let usuarios = self.leer_archivo_usuarios()?;
        self.usuarios = usuarios;
        Ok(())
    }
}

#[cfg(test)]
mod test_fm {
    use std::collections::HashMap;
    use super::*;

    const TEST_FAKE_PATH: &str = "Z:/CalleFalsa123/AvenidaSiempreviva742/";
    const TEST_FILE_NAME: &str = "streamingrust_testfile.json";

    fn usuarios_mock() -> Usuarios {
        let sus_1 = Suscripcion {
            id: 0,
            tipo: TipoSuscripcion::Basic,
            costo_mensual: 0.0.as_precio(),
            fecha_inicio: Fecha::default(),
        };
        let sus_2 = Suscripcion {
            id: 1,
            tipo: TipoSuscripcion::Classic,
            costo_mensual: 0.0.as_precio(),
            fecha_inicio: Fecha::default(),
        };
        let sus_3 = Suscripcion {
            id: 2,
            tipo: TipoSuscripcion::Super,
            costo_mensual: 0.0.as_precio(),
            fecha_inicio: Fecha::default(),
        };
        let sus_4 = Suscripcion {
            id: 3,
            tipo: TipoSuscripcion::Classic,
            costo_mensual: 0.0.as_precio(),
            fecha_inicio: Fecha::default(),
        };

        let user_1 = Usuario {
            id: 0,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::Efectivo,
            suscripcion_activa: Some(sus_1.id),
            historial_suscripciones: HashMap::from([(sus_1.id, sus_1)]),
        };
        let user_2 = Usuario {
            id: 1,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::MercadoPago(1),
            suscripcion_activa: None,
            historial_suscripciones: HashMap::from([(sus_2.id, sus_2), (sus_3.id, sus_3)]),
        };
        let user_3 = Usuario {
            id: 2,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::Debito(123),
            suscripcion_activa: None,
            historial_suscripciones: HashMap::from([(sus_4.id, sus_4)]),
        };
        let user_4 = Usuario {
            id: 3,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::MercadoPago(1),
            suscripcion_activa: None,
            historial_suscripciones: HashMap::default(),
        };

        Usuarios::from([
            (user_1.id, user_1),
            (user_2.id, user_2),
            (user_3.id, user_3),
            (user_4.id, user_4),
        ])
    }

    fn escribir_archivo_usuarios(file_name: &str) -> Result<(), ErrorSobreescribirArchivo> {
        let test_usuarios = usuarios_mock();
        sobreescribir_archivo(format!("{BASE_FOLDER}{file_name}"), &test_usuarios)
    }

    #[test]
    fn test_sobreescribir_archivo() {
        let test_data = usuarios_mock();

        let result_fail = sobreescribir_archivo(format!("{TEST_FAKE_PATH}{TEST_FILE_NAME}"), &test_data);
        let result_good = sobreescribir_archivo(format!("{BASE_FOLDER}{TEST_FILE_NAME}"), &test_data);

        if let Ok(()) = result_fail { panic!("Shouldn't be success. Fake path does not exist!") }

        match result_good {
            Ok(()) => {}
            _ => { panic!("Shouldn't throw error") }
        }
    }

    #[test]
    fn test_leer_archivo() {
        // inicializar ambos libros con funciones ya testeadas
        escribir_archivo_usuarios(TEST_FILE_NAME).expect("Shouldn't throw error");


        let Ok(value_good) = leer_archivo(format!("{BASE_FOLDER}{TEST_FILE_NAME}"))
        else { panic!("Shouldn't throw an error") };

        let Err(value_fail) = leer_archivo(format!("{TEST_FAKE_PATH}{TEST_FILE_NAME}"))
        else { panic!("Should throw error.") };

        let result_good: Usuarios = match leer_archivo_parsed::<Usuarios>(value_good.clone()) {
            Ok(res) => { res }
            _ => { panic!("Shouln't throw an error") }
        };
    }

    #[test]
    fn test_archivo_filepath() {
        assert_eq!(
            archivo_filepath(TEST_FILE_NAME),
            format!("{BASE_FOLDER}{TEST_FILE_NAME}.json")
        );
    }

    #[test]
    fn test_error_comparisons() {
        // Test ResultSobreescribirArchivo equality
        let io_error1 = ErrorSobreescribirArchivo::IOError;
        let io_error2 = ErrorSobreescribirArchivo::IOError;

        assert_eq!(io_error1, io_error2);

        // Test ErrorLeerArchivo equality
        let io_error1 = ErrorLeerArchivo::IOError;
        let io_error2 = ErrorLeerArchivo::IOError;

        assert_eq!(io_error1, io_error2);
    }

    #[test]
    fn test_integration_read_write() {
        // Write test data
        let write_result_libros = escribir_archivo_usuarios(TEST_FILE_NAME);
        assert!(write_result_libros.is_ok());

        // Read and verify libros
        let Ok(value) = leer_archivo(format!("{BASE_FOLDER}{TEST_FILE_NAME}")) else {
            panic!("Failed to read libros file");
        };

        let read_data: Usuarios = leer_archivo_parsed(value)
            .expect("Failed to parse libros data");

        assert_eq!(read_data, usuarios_mock());
    }

    #[test]
    fn test_file_management() {
        // First ensure we have test data in files
        let Ok(mut sr) = StreamingRust::new(TEST_FILE_NAME, Some(usuarios_mock()))
        else { panic!("Shouldn't throw an error") };

        // Test leer_archivo_libros()
        let result_leer = sr.leer_archivo_usuarios();
        assert!(result_leer.is_ok(), "Should successfully read usuarios file");
        let usuarios = result_leer.unwrap();
        assert_eq!(usuarios, usuarios_mock(), "Read usuarios should match test data");

        // Test actualizar_variable_usuarios()
        assert!(sr.actualizar_variable_usuarios().is_ok(),
                "Should successfully update libros");
        assert_eq!(sr.usuarios, usuarios_mock(),
                   "Updated libros should match test data");

        // Test updating with invalid JSON data
        // Write invalid JSON to test files

        std::fs::write(
            format!("{BASE_FOLDER}{TEST_FILE_NAME}.json"),
            "{invalid json"
        ).expect("Failed to write invalid JSON");

        let result_invalid = sr.actualizar_variable_usuarios();
        assert!(result_invalid.is_err(), "Should return error when parsing invalid JSON");
    }
}

//
// streamingrust.rs
//

type Usuarios = BTreeMap<u64, Usuario>;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StreamingRust {
    pub usuarios: Usuarios,
    pub file_name: String,
}
impl PartialEq for StreamingRust {
    fn eq(&self, other: &Self) -> bool {
        if self.usuarios.len() == other.usuarios.len() {
            self.usuarios.iter().all(|(id, user)| {
                let Some(other_user) = other.usuarios.get(id)
                else { return false };

                user == other_user
            });
        }

        true
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorRegistrarUsuario {
    UsuarioYaExiste,
    Archivo(ErrorSobreescribirArchivo)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorMejorarSuscripcion {
    UsuarioInexistente,
    SuscripcionMaxima,
    SinSuscripcion,
    Archivo(ErrorSobreescribirArchivo)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorDegradarSuscripcion {
    UsuarioInexistente,
    SuscripcionMinima,
    SinSuscripcion,
    Archivo(ErrorSobreescribirArchivo)
}

impl StreamingRust {
    pub fn new(file_name: &str, usuarios: Option<Usuarios>) -> Result<StreamingRust, ErrorSobreescribirArchivo> {
        let sr = StreamingRust { usuarios: usuarios.unwrap_or_leer(file_name), file_name: file_name.to_string() };
        sr.sobreescribir_archivo_usuarios(file_name)?;
        Ok(sr)
    }

    pub fn registrar_usuario(&mut self, user: Usuario) -> Result<(), ErrorRegistrarUsuario> {
        if self.usuarios.contains_key(&user.id) {
            return Err(ErrorRegistrarUsuario::UsuarioYaExiste)
        }

        self.usuarios.insert(user.id, user);
        match self.sobreescribir_archivo_usuarios(DEFAULT_FILE_NAME) {
            Ok(()) => { Ok(()) }
            Err(error) => { Err(ErrorRegistrarUsuario::Archivo(error)) }
        }
    }

    pub fn mejorar_suscripcion(&mut self, user_id: u64) -> Result<TipoSuscripcion, ErrorMejorarSuscripcion> {
        let Some(user) = self.usuarios.get_mut(&user_id)
        else { return Err(ErrorMejorarSuscripcion::UsuarioInexistente) };

        let Some(sus) = user.obtener_suscripcion()
        else { return Err(ErrorMejorarSuscripcion::SinSuscripcion) };

        match sus.mejorar() {
            Ok(nue_sus) => {
                match self.sobreescribir_archivo_usuarios(DEFAULT_FILE_NAME) {
                    Ok(()) => { Ok(nue_sus) }
                    Err(error) => { Err(ErrorMejorarSuscripcion::Archivo(error)) }
                }
            }
            Err(err) => { Err(err) }
        }
    }

    pub fn degradar_suscripcion(&mut self, user_id: u64) -> Result<TipoSuscripcion, ErrorDegradarSuscripcion> {
        let Some(user) = self.usuarios.get_mut(&user_id)
        else { return Err(ErrorDegradarSuscripcion::UsuarioInexistente) };

        let Some(sus) = user.obtener_suscripcion()
        else { return Err(ErrorDegradarSuscripcion::SinSuscripcion) };

        match sus.degradar() {
            Ok(nue_sus) => {
                match self.sobreescribir_archivo_usuarios(DEFAULT_FILE_NAME) {
                    Ok(()) => { Ok(nue_sus) }
                    Err(error) => {
                        // problema: el cambio está hecho localmente, no remotamente.
                        // posible solución: implementar una forma de "deshacer el cambio".
                        Err(ErrorDegradarSuscripcion::Archivo(error))
                    }
                }
            }
            Err(err) => { Err(err) }
        }
    }

    pub fn contabilizar_medios_de_pago(&self, contabilizar_inactivos: bool) -> HashMap<Discriminant<MedioDePago>, u32> {
        let mut contabilizador_mdp: HashMap<Discriminant<MedioDePago>, u32> = HashMap::new();

        self.usuarios.iter().for_each(|(_, usuario)| {

            if contabilizar_inactivos || usuario.suscripcion_activa.is_some() {
                if let Some(cant) = contabilizador_mdp.get_mut(&discriminant(&usuario.medio_de_pago)) {
                    *cant+= 1;
                } else {
                    contabilizador_mdp.insert(discriminant(&usuario.medio_de_pago), 1);
                }
            }

        });

        contabilizador_mdp
    }

    pub fn contabilizar_tipos_de_suscripcion(&self, contabilizar_inactivos: bool) -> HashMap<TipoSuscripcion, u32> {
        let mut contabilizador_ts: HashMap<TipoSuscripcion, u32> = HashMap::new();

        self.usuarios.iter().for_each(|(_, usuario)| {
            if contabilizar_inactivos || usuario.suscripcion_activa.is_some() {
                usuario.historial_suscripciones.iter().for_each(| (_, suscripcion) | {
                    if let Some(cant) = contabilizador_ts.get_mut(&suscripcion.tipo) {
                        *cant+= 1;
                    } else {
                        contabilizador_ts.insert(suscripcion.tipo, 1);
                    }
                });
            }
        });

        contabilizador_ts
    }

    // ➢ Saber el medio de pago que es más utilizado (opcion: por los usuarios sobre las suscripciones activas.)
    pub fn medio_de_pago_mas_utilizado(&self, contabilizar_inactivos: bool) -> Option<Discriminant<MedioDePago>> {
        //if self.usuarios.is_empty() { return None }

        let contabilizador_mdp = self.contabilizar_medios_de_pago(contabilizar_inactivos);

        contabilizador_mdp.iter()
            .max_by_key(|(_, c)| *c)
            .map(|(mdp, _)| mdp).copied()
    }

    // ➢ Saber cual es la suscripción más contratada (opcion: por los usuarios sobre las suscripciones activas.)
    pub fn suscripcion_mas_contratada(&self, contabilizar_inactivos: bool) -> Option<TipoSuscripcion> {
        if self.usuarios.is_empty() { return None }

        let contabilizador_ts = self.contabilizar_tipos_de_suscripcion(contabilizar_inactivos);

        contabilizador_ts.iter()
            .max_by_key(|(_, c)| *c)
            .map(|(ts, _)| ts).copied()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap};
    use std::fs;
    use std::mem::discriminant;
    use super::*;

    const TEST_FILE_NAME: &str = "streamingrust_testfile";

    #[test]
    fn test_new() {
        let sr1 = StreamingRust::new(TEST_FILE_NAME, None).expect("Shouldn't be an error");
        let sr2 = StreamingRust::default();

        assert_eq!(sr1, sr2);
        let sr1 = StreamingRust::new(TEST_FILE_NAME, Some(Usuarios::default())).expect("Shouldn't be an error");
        assert_eq!(sr1, sr2);
    }

    // basic, classic(none), super(none), classic(none)
    // res tipo(false) -> basic
    // res tipo(true) -> classic

    // efectivo, mpago(none), mpago(none), debito(none)
    // res mdp(false) -> efectivo
    // res mdp(true) -> mpago
    fn streamingrust_mock(delete_file: bool) -> StreamingRust {
        if delete_file {
            delete_streamingrust_mock_json();
        }
        let sus_1 = Suscripcion {
            id: 0,
            tipo: TipoSuscripcion::Basic,
            costo_mensual: 0.0.as_precio(),
            fecha_inicio: Fecha::default(),
        };
        let sus_2 = Suscripcion {
            id: 1,
            tipo: TipoSuscripcion::Classic,
            costo_mensual: 0.0.as_precio(),
            fecha_inicio: Fecha::default(),
        };
        let sus_3 = Suscripcion {
            id: 2,
            tipo: TipoSuscripcion::Super,
            costo_mensual: 0.0.as_precio(),
            fecha_inicio: Fecha::default(),
        };
        let sus_4 = Suscripcion {
            id: 3,
            tipo: TipoSuscripcion::Classic,
            costo_mensual: 0.0.as_precio(),
            fecha_inicio: Fecha::default(),
        };

        let user_1 = Usuario {
            id: 0,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::Efectivo,
            suscripcion_activa: Some(sus_1.id),
            historial_suscripciones: HashMap::from([(sus_1.id, sus_1)]),
        };
        let user_2 = Usuario {
            id: 1,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::MercadoPago(1),
            suscripcion_activa: None,
            historial_suscripciones: HashMap::from([(sus_2.id, sus_2), (sus_3.id, sus_3)]),
        };
        let user_3 = Usuario {
            id: 2,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::Debito(123),
            suscripcion_activa: None,
            historial_suscripciones: HashMap::from([(sus_4.id, sus_4)]),
        };
        let user_4 = Usuario {
            id: 3,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::MercadoPago(1),
            suscripcion_activa: None,
            historial_suscripciones: HashMap::default(),
        };

        StreamingRust::new(
            TEST_FILE_NAME,
            Some(Usuarios::from([
                (user_1.id, user_1),
                (user_2.id, user_2),
                (user_3.id, user_3),
                (user_4.id, user_4)
            ])),
        ).expect("Shouldn't throw an error.")
    }

    fn delete_streamingrust_mock_json() -> bool {
        fs::remove_file(
            format!("{BASE_FOLDER}{TEST_FILE_NAME}")
        ).is_ok()
    }

    #[test]
    fn test_estadistica_sus_mdp() {
        let sr = streamingrust_mock(false);

        // basic, classic(none), super(none), classic(none)
        // res tipo(false) -> basic
        // res tipo(true) -> classic

        assert_eq!(sr.suscripcion_mas_contratada(false), Some(TipoSuscripcion::Basic));
        assert_eq!(sr.suscripcion_mas_contratada(true), Some(TipoSuscripcion::Classic));

        // efectivo, mpago(none), mpago(none), debito(none)
        // res mdp(false) -> efectivo
        // res mdp(true) -> mpago

        assert_eq!(sr.medio_de_pago_mas_utilizado(false), Some(discriminant(&MedioDePago::Efectivo)));
        assert_eq!(sr.medio_de_pago_mas_utilizado(true), Some(discriminant(&MedioDePago::MercadoPago(0))));
    }

    #[test]
    fn test_registrar_usuario() {
        let mut sr = streamingrust_mock(true);

        let user_1 = Usuario {
            id: 0,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::Efectivo,
            suscripcion_activa: None,
            historial_suscripciones: HashMap::new(),
        };

        let user_2 = Usuario {
            id: 13548,
            email: "asd".to_string(),
            medio_de_pago: MedioDePago::Efectivo,
            suscripcion_activa: None,
            historial_suscripciones: HashMap::new(),
        };

        let result_ins1 = sr.registrar_usuario(user_1);
        let result_ins2 = sr.registrar_usuario(user_2);

        assert_eq!(result_ins1, Err(ErrorRegistrarUsuario::UsuarioYaExiste), "El usuario ya existe");
        assert_eq!(result_ins2, Ok(()), "El usuario no existe");
    }

    #[test]
    fn test_mejorar_degradar_suscripcion() {
        let mut sr = streamingrust_mock(true);

        // user id 0 tiene suscripcion BASIC
        let res = sr.degradar_suscripcion(0);
        assert_eq!(res, Err(ErrorDegradarSuscripcion::SuscripcionMinima), "La suscripción ya es lo más baja posible");

        let res = sr.mejorar_suscripcion(0);
        assert_eq!(res, Ok(TipoSuscripcion::Classic), "Debe mejorar a Classic");

        let res = sr.mejorar_suscripcion(0);
        assert_eq!(res, Ok(TipoSuscripcion::Super), "Debe mejorar a Super");

        let res = sr.mejorar_suscripcion(0);
        assert_eq!(res, Err(ErrorMejorarSuscripcion::SuscripcionMaxima), "La suscripción ya es lo más alta posible");

        let res = sr.mejorar_suscripcion(13548);
        assert_eq!(res, Err(ErrorMejorarSuscripcion::UsuarioInexistente), "ID 13548 no existe");

        let res = sr.degradar_suscripcion(13548);
        assert_eq!(res, Err(ErrorDegradarSuscripcion::UsuarioInexistente), "ID 13548 no existe");

        let res = sr.mejorar_suscripcion(1);
        assert_eq!(res, Err(ErrorMejorarSuscripcion::SinSuscripcion), "ID 1 no tiene suscripción");

        let res = sr.degradar_suscripcion(1);
        assert_eq!(res, Err(ErrorDegradarSuscripcion::SinSuscripcion), "ID 1 no tiene suscripción");
    }

}