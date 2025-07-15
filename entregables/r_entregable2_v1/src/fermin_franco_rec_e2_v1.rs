


//
// fecha.rs
//

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::BTreeMap;
use std::{fmt, fs, io};
use std::collections::btree_map::Entry::Vacant;
use std::error::Error;
use std::fs::File;
use std::io::Read;
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
mod test {
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
}

//
// libro.rs
//

/// De cada libro se conoce:
///     el título,
///     autor,
///     número de páginas,
///     género (novela, infantil, técnico, otros).
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, Debug)]
pub(crate) struct Libro {
    pub isbn: u64,
    pub titulo: String,
    pub autor: String,
    pub paginas: u16,
    pub genero: Genero,
    pub stock: u32
}

// impl<'de: 'a, 'a> Deserialize<'de> for Libro<'a> {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>
//     {
//         deserializer.deserialize_str();
//     }
// }

#[derive(Serialize, Deserialize, Default, Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum Genero {
    Novela, Infantil, Tecnico, #[default] Otros
}

impl Libro {
    pub fn new(isbn: u64, titulo: String, autor: String, paginas: u16, genero: Genero, stock: u32) -> Self {
        Self { isbn, titulo, autor, paginas, genero, stock }
    }
}

//
// prestamo.rs
//

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

//
// cliente.rs
//

/// Del cliente se conoce:
///     el nombre,
///     teléfono
///     y dirección de correo electrónico.
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, Debug)]
pub struct Cliente {
    pub id: u32,
    pub nombre: String,
    pub telefono: String,
    pub email: String,
}

impl Cliente {
    pub fn new(id: u32, nombre: String, telefono: String, email: String) -> Cliente {
        Cliente { id, nombre, telefono, email }
    }
}

//
// biblioteca_fm.rs
//

#[derive(Debug, Clone, PartialEq)]
enum DataBiblioteca<'a> {
    Libros (&'a Libros),
    Clientes (&'a Clientes),
}

impl<'a> DataBiblioteca<'a> {
    fn is_libros(&self) -> bool {
        matches!(self,DataBiblioteca::Libros(_))
    }

    fn is_prestamos(&self) -> bool {
        !self.is_libros()
    }

    fn unwrap_libros(self) -> &'a Libros {
        match self {
            DataBiblioteca::Libros(data) => { data }
            DataBiblioteca::Clientes(_) => { panic!("Self matches DataBiblioteca::Clientes, but DataBiblioteca::Libros was required.") },
        }
    }

    fn unwrap_prestamos(self) -> &'a Clientes {
        match self {
            DataBiblioteca::Libros(_) => { panic!("Self matches DataBiblioteca::Libros, but DataBiblioteca::Clientes was required.") },
            DataBiblioteca::Clientes(data) => { data },
        }
    }
}

#[derive(Debug, Clone)]
pub enum ResultSobreescribirArchivo {
    Success,
    IOError, //io::Error == ()
    SerializationError,
}

#[derive(Debug, Clone)]
pub enum ErrorLeerArchivo {
    IOError,
    DeserializationError,
}

impl PartialEq for ResultSobreescribirArchivo {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl PartialEq for ErrorLeerArchivo {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

const BASE_FOLDER: &str = "";
const FILE_NAME_FORMAT: &str = "{}_{}.json"; // {biblioteca.nombre}_{libros/prestamos}.json
const LIBROS_FILENAME: &str = "libros";
const CLIENTES_FILENAME: &str = "clientes";
fn archivo_filepath(library_name: &str, file_kind: &str) -> String {
    format!("{BASE_FOLDER}{library_name}_{file_kind}.json")
}

//
// IO part
//

fn sobreescribir_archivo(file_abs_path: String, data: &DataBiblioteca) -> ResultSobreescribirArchivo {
    let json_string = match data {
        DataBiblioteca::Libros(data) => { serde_json::to_string_pretty(data) },
        DataBiblioteca::Clientes(data) => { serde_json::to_string_pretty(data) }
    };

    let text = match json_string {
        Ok(text) => { text }
        Err(error) => { return ResultSobreescribirArchivo::SerializationError }
    };

    match fs::write(file_abs_path, text) {
        Err(error) => ResultSobreescribirArchivo::IOError,
        _ => ResultSobreescribirArchivo::Success
    }
}

fn leer_archivo(filepath: String) -> Result<Value, ErrorLeerArchivo> {
    let mut file = match File::open(filepath) {
        Ok(file) => { file }
        Err(error) => { return Err(ErrorLeerArchivo::IOError) }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(error) => return Err(ErrorLeerArchivo::IOError)
    }

    match serde_json::from_str(&contents) {
        Err(error) => Err(ErrorLeerArchivo::DeserializationError),
        Ok(value) => Ok(value),
    }
}

fn leer_archivo_parsed<T>(value: Value) -> Result<T, ErrorLeerArchivo> where T: DeserializeOwned {
    let data: T = match serde_json::from_value::<T>(value) {
        Ok(data) => { data }
        Err(error) => { return Err(ErrorLeerArchivo::DeserializationError) }
    };

    Ok(data)
}

//
// Implementation / Serialization-Deserialization part
//

pub trait BibliotecaFileManagement {
    fn sobreescribir_archivo_libros(&self) -> ResultSobreescribirArchivo;
    fn sobreescribir_archivo_clientes(&self) -> ResultSobreescribirArchivo;
    fn leer_archivo_libros(&self) -> Result<Libros, ErrorLeerArchivo>;
    fn leer_archivo_clientes(&self) -> Result<Clientes, ErrorLeerArchivo>;
    fn actualizar_variable_libros(&mut self) -> Option<ErrorLeerArchivo>;
    fn actualizar_variable_clientes(&mut self) -> Option<ErrorLeerArchivo>;
}

impl BibliotecaFileManagement for Biblioteca {
    fn sobreescribir_archivo_libros(&self) -> ResultSobreescribirArchivo {
        let data = DataBiblioteca::Libros(&self.libros);
        sobreescribir_archivo(archivo_filepath(&self.nombre, LIBROS_FILENAME), &data)
    }

    fn sobreescribir_archivo_clientes(&self) -> ResultSobreescribirArchivo {
        let data = DataBiblioteca::Clientes(&self.clientes);
        sobreescribir_archivo(archivo_filepath(&self.nombre, CLIENTES_FILENAME), &data)
    }

    fn leer_archivo_libros(&self) -> Result<Libros, ErrorLeerArchivo> {
        let lectura = leer_archivo(archivo_filepath(&self.nombre, LIBROS_FILENAME))?;
        let libros: Libros = leer_archivo_parsed(lectura)?;
        Ok(libros)
    }

    fn leer_archivo_clientes(&self) -> Result<Clientes, ErrorLeerArchivo> {
        let lectura = leer_archivo(archivo_filepath(&self.nombre, CLIENTES_FILENAME))?;
        let clientes: Clientes = leer_archivo_parsed(lectura)?;
        Ok(clientes)
    }

    fn actualizar_variable_libros(&mut self) -> Option<ErrorLeerArchivo> {
        let libros = match self.leer_archivo_libros() {
            Ok(libros) => libros,
            Err(error) => return Some(error)
        };

        self.libros = libros;
        None
    }

    fn actualizar_variable_clientes(&mut self) -> Option<ErrorLeerArchivo> {
        let clientes = match self.leer_archivo_clientes() {
            Ok(clientes) => clientes,
            Err(error) => return Some(error)
        };

        self.clientes = clientes;
        None
    }
}

#[cfg(test)]
mod tests_fm {
    use super::*;

    const TEST_FAKE_PATH: &str = "Z:/CalleFalsa123/AvenidaSiempreviva742/";
    const TEST_LIBROS_FILE_NAME: &str = "libros_testfile.json";
    const TEST_CLIENTES_FILE_NAME: &str = "clientes_testfile.json";

    fn libro_economia_1() -> Libro {
        Libro::new(
            1,
            "Economía en una lección".to_string(),
            "xd".to_string(),
            1,
            Genero::Tecnico,
            1
        )
    }
    fn libro_xd_2() -> Libro {
        let mut libro = Libro::default();
        libro.isbn = 2;
        libro.titulo = "xd".to_string();
        libro.stock = 2;
        libro
    }
    fn libro_harrypotter_3() -> Libro {
        let mut libro = Libro::default();
        libro.isbn = 3;
        libro.titulo = "Harry Potter y qsy q mas".to_string();
        libro.stock = 3;
        libro
    }
    fn testdata_libros() ->  Libros {
        Libros::from(
            [(1, libro_economia_1()),
                (2, libro_xd_2()),
                (3, libro_harrypotter_3())]
        )
    }

    fn cliente_pepe_1() -> Cliente {
        Cliente::new(
            1,
            "pepe".to_string(),
            "123".to_string(),
            "pepe@gmail.com".to_string()
        )
    }
    fn cliente_manuel_2() -> Cliente {
        Cliente::new(
            2,
            "manuel".to_string(),
            "123".to_string(),
            "manuel@gmail.com".to_string()
        )
    }
    fn prestamo_1_1() -> Prestamo {
        Prestamo::new(
            1,
            1,
            Fecha { dia: 22, mes: 8, ano: 2002 },
            EstadoPrestamo::Prestando
        )
    }
    fn prestamo_2_2() -> Prestamo {
        Prestamo::new(
            2,
            2,
            Fecha { dia: 23, mes: 6, ano: 2025 },
            EstadoPrestamo::Prestando
        )
    }
    fn prestamo_3_2() -> Prestamo {
        Prestamo::new(
            3,
            2,
            Fecha { dia: 1, mes: 1, ano: 2025 },
            EstadoPrestamo::Devuelto( Fecha { dia: 19, mes: 11, ano: 2023 } )
        )
    }
    fn testdata_clientes() -> Clientes {
        Clientes::from([
            (1, (cliente_pepe_1(), vec![prestamo_1_1()])),
            (2, (cliente_manuel_2(), vec![prestamo_2_2(), prestamo_3_2()]))
        ])
    }

    fn escribir_archivo_libros() -> ResultSobreescribirArchivo {
        let test_libros = testdata_libros();
        let test_data = DataBiblioteca::Libros(&test_libros);
        sobreescribir_archivo(format!("{BASE_FOLDER}{TEST_LIBROS_FILE_NAME}"), &test_data)
    }
    fn escribir_archivo_clientes() -> ResultSobreescribirArchivo {
        let test_clientes = testdata_clientes();
        let test_data = DataBiblioteca::Clientes(&test_clientes);
        sobreescribir_archivo(format!("{BASE_FOLDER}{TEST_CLIENTES_FILE_NAME}"), &test_data)
    }

    #[test]
    fn test_escribir_archivo() {
        // libros

        let test_libros = testdata_libros();
        let test_libros_data = DataBiblioteca::Libros(&test_libros);

        let result_libros_fail = sobreescribir_archivo(format!("{TEST_FAKE_PATH}{TEST_LIBROS_FILE_NAME}"), &test_libros_data);
        let result_libros_good = escribir_archivo_libros();

        if let ResultSobreescribirArchivo::Success = result_libros_fail { panic!("Shouldn't be success. Fake path does not exist!") }

        match result_libros_good {
            ResultSobreescribirArchivo::Success => {}
            _ => { panic!("Shouldn't throw error") }
        }

        // clientes

        let test_clientes = testdata_libros();
        let test_clientes_data = DataBiblioteca::Libros(&test_clientes);

        let result_clientes_fail = sobreescribir_archivo(format!("{TEST_FAKE_PATH}{TEST_LIBROS_FILE_NAME}"), &test_libros_data);
        let result_clientes_good = escribir_archivo_clientes();

        if let ResultSobreescribirArchivo::Success = result_clientes_fail { panic!("Shouldn't be success. Fake path does not exist!") }

        match result_clientes_good {
            ResultSobreescribirArchivo::Success => {}
            _ => { panic!("Shouldn't throw error") }
        }
    }

    #[test]
    fn test_leer_archivo() {
        // inicializar ambos libros con funciones ya testeadas
        escribir_archivo_libros();
        escribir_archivo_clientes();

        // libros

        let Ok(value_libros_good) = leer_archivo(format!("{BASE_FOLDER}{TEST_LIBROS_FILE_NAME}"))
        else { panic!("Shouldn't throw an error") };

        let Err(value_libros_fail) = leer_archivo(format!("{TEST_FAKE_PATH}{TEST_LIBROS_FILE_NAME}"))
        else { panic!("Should throw error.") };

        let result_libros_good: Libros = match leer_archivo_parsed::<Libros>(value_libros_good.clone()) {
            Ok(res) => { res }
            _ => { panic!("Shouln't throw an error") }
        };

        assert!(leer_archivo_parsed::<Clientes>(value_libros_good).is_err(), "Should throw an error");

        // clientes

        let Ok(value_clientes_good) = leer_archivo(format!("{BASE_FOLDER}{TEST_CLIENTES_FILE_NAME}"))
        else { panic!("Shouldn't throw an error") };

        let Err(value_clientes_fail) = leer_archivo(format!("{TEST_FAKE_PATH}{TEST_CLIENTES_FILE_NAME}"))
        else { panic!("Should throw error.") };

        let result_clientes_good: Clientes = match leer_archivo_parsed::<Clientes>(value_clientes_good.clone()) {
            Ok(res) => { res }
            _ => { panic!("Shouln't throw an error") }
        };

        assert!(leer_archivo_parsed::<Libros>(value_clientes_good).is_err(), "Should throw an error");
    }

    #[test]
    fn test_archivo_filepath() {
        let library_name = "test_library";

        assert_eq!(
            archivo_filepath(library_name, TEST_LIBROS_FILE_NAME),
            format!("{BASE_FOLDER}{library_name}_{TEST_LIBROS_FILE_NAME}.json")
        );
        assert_eq!(
            archivo_filepath(library_name, TEST_CLIENTES_FILE_NAME),
            format!("{BASE_FOLDER}{library_name}_{TEST_CLIENTES_FILE_NAME}.json")
        );
    }

    #[test]
    fn test_data_biblioteca_methods() {
        let test_libros = testdata_libros();
        let test_clientes = testdata_clientes();

        let data_libros = DataBiblioteca::Libros(&test_libros);
        let data_clientes = DataBiblioteca::Clientes(&test_clientes);

        // Test is_libros()
        assert!(data_libros.is_libros());
        assert!(!data_clientes.is_libros());

        // Test is_prestamos()
        assert!(data_clientes.is_prestamos());
        assert!(!data_libros.is_prestamos());

        // Test unwrap_libros()
        assert_eq!(data_libros.clone().unwrap_libros(), &test_libros);

        // Test unwrap_prestamos()
        assert_eq!(data_clientes.clone().unwrap_prestamos(), &test_clientes);

        // Test unwrap panics
        let data_libros_clone = data_libros.clone();
        let data_clientes_clone = data_clientes.clone();

        std::panic::catch_unwind(move || {
            data_libros_clone.unwrap_prestamos();
        }).expect_err("Should panic when unwrapping libros as prestamos");

        std::panic::catch_unwind(move || {
            data_clientes_clone.unwrap_libros();
        }).expect_err("Should panic when unwrapping prestamos as libros");
    }

    #[test]
    fn test_error_comparisons() {
        // Test ResultSobreescribirArchivo equality
        let success1 = ResultSobreescribirArchivo::Success;
        let success2 = ResultSobreescribirArchivo::Success;
        let io_error = ResultSobreescribirArchivo::IOError;

        assert_eq!(success1, success2);
        assert_ne!(success1, io_error);

        // Test ErrorLeerArchivo equality
        let io_error1 = ErrorLeerArchivo::IOError;
        let io_error2 = ErrorLeerArchivo::IOError;

        assert_eq!(io_error1, io_error2);
    }

    #[test]
    fn test_integration_read_write() {
        // Write test data
        let write_result_libros = escribir_archivo_libros();
        assert!(matches!(write_result_libros, ResultSobreescribirArchivo::Success));

        let write_result_clientes = escribir_archivo_clientes();
        assert!(matches!(write_result_clientes, ResultSobreescribirArchivo::Success));

        // Read and verify libros
        let Ok(libros_value) = leer_archivo(format!("{BASE_FOLDER}{TEST_LIBROS_FILE_NAME}")) else {
            panic!("Failed to read libros file");
        };
        let read_libros: Libros = leer_archivo_parsed(libros_value)
            .expect("Failed to parse libros data");

        assert_eq!(read_libros, testdata_libros());

        // Read and verify clientes
        let Ok(clientes_value) = leer_archivo(format!("{BASE_FOLDER}{TEST_CLIENTES_FILE_NAME}")) else {
            panic!("Failed to read clientes file");
        };
        let read_clientes: Clientes = leer_archivo_parsed(clientes_value)
            .expect("Failed to parse clientes data");

        assert_eq!(read_clientes, testdata_clientes());
    }

    #[test]
    fn test_biblioteca_file_management() {
        // First ensure we have test data in files

        let mut biblioteca = Biblioteca::new(
            "testlibrary".to_string(),
            "test address".to_string(),
            Some(testdata_libros()),
            Some(testdata_clientes())
        );

        // Test leer_archivo_libros()
        let result_libros = biblioteca.leer_archivo_libros();
        assert!(result_libros.is_ok(), "Should successfully read libros file");
        let libros = result_libros.unwrap();
        assert_eq!(libros, testdata_libros(), "Read libros should match test data");


        // Test actualizar_variable_libros()
        assert!(biblioteca.actualizar_variable_libros().is_none(),
                "Should successfully update libros");
        assert_eq!(biblioteca.libros, testdata_libros(),
                   "Updated libros should match test data");

        // Test actualizar_variable_clientes()
        assert!(biblioteca.actualizar_variable_clientes().is_none(),
                "Should successfully update clientes");
        assert_eq!(biblioteca.clientes, testdata_clientes(),
                   "Updated clientes should match test data");

        // Test updating with invalid JSON data
        // Write invalid JSON to test files

        // libros
        std::fs::write(
            format!("{BASE_FOLDER}testlibrary_libros.json"),
            "{invalid json"
        ).expect("Failed to write invalid JSON");

        let result_invalid = biblioteca.actualizar_variable_libros();
        assert!(result_invalid.is_some(), "Should return error when parsing invalid JSON");

        // Clean up test file
        std::fs::remove_file(format!("{BASE_FOLDER}testlibrary_libros.json"))
            .expect("Failed to clean up test file");

        std::fs::write(
            format!("{BASE_FOLDER}testlibrary_libros.json"),
            "{invalid json"
        ).expect("Failed to write invalid JSON");

        let result_invalid = biblioteca.actualizar_variable_libros();
        assert!(result_invalid.is_some(), "Should return error when parsing invalid JSON");

        // Clean up test file
        std::fs::remove_file(format!("{BASE_FOLDER}testlibrary_libros.json"))
            .expect("Failed to clean up test file");

        // clientes
        std::fs::write(
            format!("{BASE_FOLDER}testlibrary_clientes.json"),
            "{invalid json"
        ).expect("Failed to write invalid JSON");

        let result_invalid = biblioteca.actualizar_variable_clientes();
        assert!(result_invalid.is_some(), "Should return error when parsing invalid JSON");

        // Clean up test file
        std::fs::remove_file(format!("{BASE_FOLDER}testlibrary_clientes.json"))
            .expect("Failed to clean up test file");
    }
}

//
// biblioteca.rs
//

const MAX_PRESTAMOS_ACTIVOS: usize = 5;

/// # Biblioteca
///
/// `nombre: String` - Nombre de la biblioteca<br>
/// `direccion: String` - Dirección física de la biblioteca<br>
/// `libros: BTreeMap<u64, Libro>` - Libros de la biblioteca.<br>
/// `prestamos: BTreeMap<u32, (Cliente, Vec<Prestamo>)>` -> `BTreeMap<ID del cliente, (Cliente, Vec<Prestamo>)>`
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, Debug)]
pub struct Biblioteca {
    pub nombre: String,
    pub direccion: String,
    pub libros: BTreeMap<u64, Libro>,
    pub clientes: BTreeMap<u32, (Cliente, Vec<Prestamo>)> // <ID cliente, (Cliente, Vec<Préstamo>)>
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ErrorDecrementarStock {
    StockEsCero, LibroNoExiste
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ErrorIncrementarStock {
    LibroNoExiste, Overflow
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ErrorRealizarPrestamo {
    PrestamosMaximosAlcanzados, StockInsuficiente, ClienteInexistente, LibroNoExiste
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ErrorBuscarPrestamo {
    PrestamoInexistente, ClienteInexistente
}

#[derive(Debug, PartialEq)]
pub enum ErrorDevolverLibro {
    PrestamoInexistente,
    ClienteInexistente,
    LibroYaDevuelto
}

#[derive(Debug)]
pub enum ResultRegistrarLibro {
    Exito{ resultado_fm: ResultSobreescribirArchivo },
    LibroYaExiste,
}

#[derive(Debug)]
pub enum ResultRegistrarCliente {
    Exito{ resultado_fm: ResultSobreescribirArchivo },
    ClienteYaExiste,
}

// some functions can have two kinds of erros: FileManagement (write in all cases) and Local errors.
// specifically, the functions that make use if the File Management system.
pub enum DoubleError<T> {
    LocalError(T),
    RemoteError(ResultSobreescribirArchivo),
}

impl<T> DoubleError<T> {
    pub fn is_local(&self) -> bool {
        matches!(self, DoubleError::LocalError(_))
    }
    pub fn is_remote(&self) -> bool {
        !self.is_local()
    }
}

impl<T> From<T> for DoubleError<T> {
    fn from(value: T) -> Self {
        DoubleError::LocalError(value)
    }
}

impl<T> DoubleError<T> {
    fn from(value: ResultSobreescribirArchivo) -> Self {
        Self::RemoteError(value)
    }
}

type Libros = BTreeMap<u64, Libro>;
type Clientes = BTreeMap<u32, (Cliente, Vec<Prestamo>)>;

impl Biblioteca {

    /// ### `fn new() -> Biblioteca`
    /// Crea una nueva instancia de biblioteca
    ///
    /// #### Recibe:<br>
    /// - `nombre` - Nombre de la biblioteca
    /// - `direccion` - Dirección de la biblioteca
    /// - `libros` - Opcional: Lista de libros de la biblioteca.
    /// - `prestamos` - Opcional: Lista de préstamos de la biblioteca<br>
    ///   * Si `libros` o `prestamos` son None, intentará leer la información individualmente de disco. De no poder, creará un conjunto vacío.
    ///   * Si `libros` o `prestamos` son Some(data), creará un nuevo archivo que contenga data.
    ///
    /// #### Devuelve:
    /// `Biblioteca` - Nueva instancia de Biblioteca
    pub fn new(nombre: String, direccion: String, libros: Option<Libros>, clientes: Option<Clientes>) -> Biblioteca {
        let mut biblioteca = Biblioteca {
            nombre,
            direccion,
            libros: Libros::new(),
            clientes: Clientes::new()
        };

        if let Some(data) = libros {
            biblioteca.libros = data;
            biblioteca.sobreescribir_archivo_libros();
        } else {
            biblioteca.libros = biblioteca.leer_archivo_libros().unwrap_or_default();
        }

        if let Some(data) = clientes {
            biblioteca.clientes = data;
            biblioteca.sobreescribir_archivo_clientes();
        } else {
            biblioteca.clientes = biblioteca.leer_archivo_clientes().unwrap_or_default();
        }

        biblioteca
    }

    pub fn registrar_libro(&mut self, libro: Libro) -> ResultRegistrarLibro {
        if let Vacant(vacant) = self.libros.entry(libro.isbn) {
            vacant.insert(libro);
            ResultRegistrarLibro::Exito { resultado_fm: self.sobreescribir_archivo_libros() }
        } else { ResultRegistrarLibro::LibroYaExiste }
    }

    pub fn registrar_cliente(&mut self, cliente: Cliente) -> ResultRegistrarCliente {
        if let Vacant(vacant) = self.clientes.entry(cliente.id) {
            vacant.insert((cliente, Vec::new()));
            ResultRegistrarCliente::Exito { resultado_fm: self.sobreescribir_archivo_clientes() }
        } else { ResultRegistrarCliente::ClienteYaExiste }
    }

    /// ### `fn cantidad_de_copias_en_stock(isbn) -> Option<u32>`
    /// Devuelve la cantidad de copias disponibles de un libro
    ///
    /// #### Recibe:<br>
    /// `isbn` - ID del libro a consultar
    ///
    /// #### Devuelve:<br>
    /// `Some(u32)` - Cantidad (u32) de libros en stock<br>
    /// `None` - No existe el libro consultado
    pub fn cantidad_de_copias_en_stock(&self, isbn: u64) -> Option<u32> {
        self.libros.get(&isbn).map(|libro| libro.stock)
    }

    /// ### `fn decrementar_stock_libro(isbn) -> Result<u32, ErrorDecrementarStock>`
    /// Devuelve la cantidad de libros en stock después de decrementarla en 1
    ///
    /// #### Recibe:<br>
    /// `isbn` - ID del libro a consultar
    ///
    /// #### Devuelve:<br>
    /// `u32` - Cantidad de libros después de decrementar<br>
    /// `ErrorDecrementarStock` - El stock es cero o el libro no existe
    pub fn decrementar_stock_libro(&mut self, isbn: u64) -> Result<u32, DoubleError<ErrorDecrementarStock>> {
        let nuevo_stock = match self.libros.get_mut(&isbn) {
            Some(libro) => {
                if libro.stock == 0 {
                    return Err(ErrorDecrementarStock::StockEsCero.into())
                }
                libro.stock-= 1;
                libro.stock
            },
            None => return Err(ErrorDecrementarStock::LibroNoExiste.into()),
        };

        match self.sobreescribir_archivo_libros() {
            ResultSobreescribirArchivo::Success => (),
            x => return Err(DoubleError::RemoteError(x))
        }

        Ok(nuevo_stock)
    }


    /// ### `fn incrementar_stock_libro(isbn) -> Result<u32, ErrorIncrementarStock>`
    /// Devuelve la cantidad de libros en stock después de incrementarla en 1
    ///
    /// #### Recibe:<br>
    /// `isbn` - ID del libro a consultar
    ///
    /// #### Devuelve:<br>
    /// `u32` - Cantidad de libros después de decrementar<br>
    /// `ErrorIncrementarStock` - El stock es `u32::MAX` o el libro no existe
    pub fn incrementar_stock_libro(&mut self, isbn: u64) -> Result<u32, DoubleError<ErrorIncrementarStock>> {
        let nuevo_stock = match self.libros.get_mut(&isbn) {
            Some(libro) => {
                if libro.stock == u32::MAX {
                    return Err(ErrorIncrementarStock::Overflow.into())
                }
                libro.stock+= 1;
                libro.stock
            },
            None => return Err(ErrorIncrementarStock::LibroNoExiste.into())
        };

        match self.sobreescribir_archivo_libros() {
            ResultSobreescribirArchivo::Success => (),
            x => return Err(DoubleError::from(x))
        }

        Ok(nuevo_stock)
    }

    /// ### `fn cantidad_prestamos_cliente(cliente) -> Option<usize>`
    /// Devuelve la cantidad de préstamos efectuados a un cliente
    ///
    /// #### Recibe:<br>
    /// `cliente` - ID del cliente a consultar<br>
    ///
    /// #### Devuelve:<br>
    /// `Some(usize)` - Cantidad de préstamos efectuados al cliente<br>
    /// `None` - El cliente no existe
    pub fn cantidad_prestamos_cliente(&self, cliente: u32) -> Option<usize> {
        self.clientes.get(&cliente).map(|cliente| cliente.1.len())
    }

    /// ### `fn cantidad_stock_libro(isbn) -> Option<u32>`
    /// Devuelve la cantidad de libros en stock del libro consultado
    ///
    /// #### Recibe:<br>
    /// `isbn` - ID del libro a consultar<br>
    ///
    /// #### Devuelve:<br>
    /// `Some(u32)` - Cantidad de libros en stock<br>
    /// `None` - El libro no existe
    pub fn cantidad_stock_libro(&self, isbn: u64) -> Option<u32> {
        self.libros.get(&isbn).map(|libro| libro.stock)
    }

    /// ### `fn realizar_prestamo(cliente, isbn, vencimiento) -> Result(usize, ErrorRealizarPrestamo)`
    /// Realiza un préstamo del libro en nombre del cliente con el vencimiento especificado
    ///
    /// #### Recibe:<br>
    /// `id_cliente` - ID del cliente a efectuar el préstamo<br>
    /// `isbn` - ID del libro a prestar<br>
    /// `vencimiento` - Fecha de vencimiento del préstamo<br>
    ///
    /// #### Devuelve:<br>
    /// `usize` - Cantidad de préstamos del cliente, incluyendo el recién realizado
    pub fn realizar_prestamo(&mut self, id_cliente: u32, isbn: u64, vencimiento: Fecha) -> Result<usize, DoubleError<ErrorRealizarPrestamo>> /* <Cant. préstamos vigentes del cliente, Error> */ {
        match self.libros.get(&isbn) {
            Some(libro) => {
                if libro.stock == 0 {
                    return Err(ErrorRealizarPrestamo::StockInsuficiente.into())
                }
            },
            None => return Err(ErrorRealizarPrestamo::LibroNoExiste.into())
        }

        // obtener cliente
        let Some(datos_cliente) = self.clientes.get_mut(&id_cliente)
        else { return Err(ErrorRealizarPrestamo::ClienteInexistente.into()) };

        // check cant. max. prestamos
        let cant_libros_no_devueltos = datos_cliente.1.iter().filter(|p| p.estado == EstadoPrestamo::Prestando).count();
        if cant_libros_no_devueltos >= MAX_PRESTAMOS_ACTIVOS {
            return Err(ErrorRealizarPrestamo::PrestamosMaximosAlcanzados.into());
        }

        // si el préstamo alguna vez se realizó: eliminarlo para reemplazarlo.
        datos_cliente.1.retain(|p| p.isbn != isbn);

        // realizar préstamo
        let prestamo = Prestamo::new(isbn, id_cliente, vencimiento, EstadoPrestamo::Prestando);
        datos_cliente.1.push(prestamo);

        // reducir stock
        if let Some(libro) = self.libros.get_mut(&isbn) {
            libro.stock-= 1;
        }

        match self.sobreescribir_archivo_libros() {
            ResultSobreescribirArchivo::Success => (),
            x => return Err(DoubleError::from(x))
        }

        match self.sobreescribir_archivo_clientes() {
            ResultSobreescribirArchivo::Success => (),
            x => return Err(DoubleError::from(x))
        }

        Ok(cant_libros_no_devueltos + 1)
    }

    /// ### `fn prestamos_a_vencer(feca_hoy, dias) -> Vec<&Prestamo>`
    /// Devuelve un Vec<&Prestamo> con los préstamos a vencer en los próximos `dias` días
    ///
    /// #### Recibe:<br>
    /// `fecha_hoy` - Fecha del día de hoy<br>
    /// `dias` - Días en los que vencerán los préstamos devueltos<br>
    ///
    /// #### Devuelve:<br>
    /// `Vec<&Prestamo>` - Los préstamos que vencerán en los próximos `dias` días
    pub fn prestamos_por_vencer(&self, fecha_hoy: Fecha, dias: u32) -> Vec<&Prestamo> {
        let mut prestamos_por_vencer: Vec<&Prestamo> = Vec::new();

        let mut fecha_limite = fecha_hoy;
        fecha_limite.sumar_dias(dias);
        let fecha_limite  = fecha_limite; // quitar mutabilidad

        for prestamos_cliente in self.clientes.values() {
            for prestamo in &prestamos_cliente.1 {

                if prestamo.estado == EstadoPrestamo::Prestando && prestamo.vencimiento <= fecha_limite {
                    prestamos_por_vencer.push(prestamo);
                }
            }
        }

        prestamos_por_vencer
    }

    /// ### `fn prestamos_vencidos(fecha_hoy) -> Vec<&Prestamo>`
    /// Devuelve los prestamos que hayan vencido
    ///
    /// #### Recibe:<br>
    /// `fecha_hoy` - La fecha de hoy<br>
    ///
    /// #### Devuelve:<br>
    /// `Vec<&Prestamo>` - Los préstamos que han vencido
    pub fn prestamos_vencidos(&self, fecha_hoy: Fecha) -> Vec<&Prestamo> {
        let mut prestamos_vencidos: Vec<&Prestamo> = Vec::new();

        for prestamos_cliente in self.clientes.values() {
            for prestamo in &prestamos_cliente.1 {
                if prestamo.estado == EstadoPrestamo::Prestando && prestamo.vencimiento < fecha_hoy {
                    prestamos_vencidos.push(prestamo);
                }
            }
        }

        prestamos_vencidos
    }

    /// ### `fn buscar_prestamo(isbn, id_cliente) -> Result<&Prestamo, ErrorBuscarPrestamo>`
    /// Devuelve un préstamo en específico
    ///
    /// #### Recibe:<br>
    /// `isbn` - ID del libro prestado<br>
    /// `id_cliente` - ID del cliente del préstamo<br>
    ///
    /// #### Devuelve:<br>
    /// `&Prestamo` - El préstamo buscado<br>
    /// `ErrorBuscarPrestamo` - El préstamo o el cliente no existen
    pub fn buscar_prestamo(&self, isbn: u64, id_cliente: u32) -> Result<&Prestamo, ErrorBuscarPrestamo> {
        match self.clientes.get(&id_cliente) {
            Some(dato) => {
                for prestamo in &dato.1 {
                    if prestamo.isbn == isbn { return Ok(prestamo) }
                }
                Err(ErrorBuscarPrestamo::PrestamoInexistente)
            },
            None => Err(ErrorBuscarPrestamo::ClienteInexistente)
        }
    }

    /// ### `fn devolver_libro(isbn, id_cliente, fecha_hoy) -> Result<&Prestamo, ErrorDevolverLibro>`
    /// Realiza la devolución del libro especificado
    ///
    /// #### Recibe:<br>
    /// `isbn` - ID del libro a devolver<br>
    /// `id_cliente` - ID del cliente que devuelve<br>
    /// `fecha_hoy` - La fecha de hoy<br>
    ///
    /// #### Devuelve:<br>
    /// `usize` - La cantidad de dicho libro en stock después de ser devuelto<br>
    /// `ErrorDevolverLibro` - El cliente o el préstamo no existen o ya fue devuelto
    pub fn devolver_libro(&mut self, isbn: u64, id_cliente: u32, fecha_hoy: Fecha) -> Result<u32, DoubleError<ErrorDevolverLibro>> {
        let Some(data_cliente) = self.clientes.get_mut(&id_cliente)
        else { return Err(ErrorDevolverLibro::ClienteInexistente.into()) };

        let Some(prestamo) = data_cliente.1.iter_mut().find(|prestamo| prestamo.isbn == isbn )
        else { return Err(ErrorDevolverLibro::PrestamoInexistente.into()) };

        if matches!(prestamo.estado, EstadoPrestamo::Devuelto(_)) {
            return Err(ErrorDevolverLibro::LibroYaDevuelto.into())
        }

        prestamo.estado = EstadoPrestamo::Devuelto(fecha_hoy);

        let stock_libro = if let Some(libro) = self.libros.get_mut(&isbn) {
            libro.stock+= 1;
            libro.stock
        } else { 0 };

        match self.sobreescribir_archivo_libros() {
            ResultSobreescribirArchivo::Success => (),
            x => return Err(DoubleError::from(x))
        }

        match self.sobreescribir_archivo_clientes() {
            ResultSobreescribirArchivo::Success => (),
            x => return Err(DoubleError::from(x))
        }

        Ok(stock_libro)
    }
}

//
// tests biblioteca
//

#[cfg(test)]
mod tests_biblioteca {
    use super::*;

    fn biblioteca_de_pepe() -> Biblioteca {
        Biblioteca::new(
            "biblio de pepe".to_string(),
            "donde queda".to_string(),
            Some(BTreeMap::from(
                [(1, libro_economia_1()),
                    (2, libro_xd_2()),
                    (3, libro_harrypotter_3()),
                    (4, libro_asd_4()),
                    (5, libro_estadistica_5()),
                    (u64::from(u32::MAX), libro_algo_u32max())])),
            None)
    }
    fn cliente_pepe() -> Cliente {
        Cliente::new(
            1,
            "pepe".to_string(),
            "123".to_string(),
            "pepe@gmail.com".to_string()
        )
    }
    fn cliente_manuel() -> Cliente {
        Cliente::new(
            3,
            "manuel".to_string(),
            "123".to_string(),
            "manuel@gmail.com".to_string()
        )
    }
    fn libro_economia_1() -> Libro {
        Libro::new(
            1,
            "Economía en una lección".to_string(),
            "xd".to_string(),
            1,
            Genero::Tecnico,
            1
        )
    }
    fn libro_xd_2() -> Libro {
        let mut libro = Libro::default();
        libro.isbn = 2;
        libro.titulo = "xd".to_string();
        libro.stock = 2;
        libro
    }
    fn libro_harrypotter_3() -> Libro {
        let mut libro = Libro::default();
        libro.isbn = 3;
        libro.titulo = "Harry Potter y qsy q mas".to_string();
        libro.stock = 3;
        libro
    }
    fn libro_asd_4() -> Libro {
        let mut libro = Libro::default();
        libro.isbn = 4;
        libro.titulo = "asd".to_string();
        libro.stock = 4;
        libro
    }
    fn libro_estadistica_5() -> Libro {
        let mut libro = Libro::default();
        libro.isbn = 5;
        libro.titulo = "Estadística".to_string();
        libro.stock = 5;
        libro
    }
    fn libro_algo_u32max() -> Libro {
        let mut libro = Libro::default();
        libro.isbn = u64::from(u32::MAX);
        libro.titulo = "algo".to_string();
        libro.stock = u32::MAX;
        libro
    }

    #[test]
    fn test_cant_copias() {
        let mut biblioteca = biblioteca_de_pepe();

        // test dec/inc libro inexistente

        let res = match biblioteca.decrementar_stock_libro(5000) {
            Ok(_) => { panic!("Debe ser error") }
            Err(err) => {err}
        };

        let res = match res {
            DoubleError::LocalError(err) => { err }
            DoubleError::RemoteError(_) => { panic!("Debe ser un error local") }
        };

        assert_eq!(res, ErrorDecrementarStock::LibroNoExiste);

        let res = match biblioteca.incrementar_stock_libro(5000) {
            Ok(_) => { panic!("Debe ser error") }
            Err(err) => {err}
        };

        let res = match res {
            DoubleError::LocalError(err) => { err }
            DoubleError::RemoteError(_) => { panic!("Debe ser un error local") }
        };

        assert_eq!(res, ErrorIncrementarStock::LibroNoExiste);

        // test dec

        assert_eq!(biblioteca.cantidad_de_copias_en_stock(5).unwrap(), 5, "ISBN 5 tiene 5 copias");
        assert_eq!(biblioteca.cantidad_de_copias_en_stock(3).unwrap(), 3, "ISBN 3 tiene 3 copias");
        assert_eq!(biblioteca.cantidad_de_copias_en_stock(1).unwrap(), 1, "ISBN 1 tiene 1 copias");

        let res1 = biblioteca.decrementar_stock_libro(5);
        let res2 = biblioteca.decrementar_stock_libro(3);
        let res3 = biblioteca.decrementar_stock_libro(1);

        assert!(res1.is_ok()); assert!(res2.is_ok()); assert!(res3.is_ok());

        assert_eq!(biblioteca.cantidad_de_copias_en_stock(5).unwrap(), 4, "ISBN 5 tiene 4 copias");
        assert_eq!(biblioteca.cantidad_de_copias_en_stock(3).unwrap(), 2, "ISBN 3 tiene 2 copias");
        assert_eq!(biblioteca.cantidad_de_copias_en_stock(1).unwrap(), 0, "ISBN 1 tiene 0 copias");

        // test inc

        let res1 = biblioteca.incrementar_stock_libro(5);
        let res2 = biblioteca.incrementar_stock_libro(3);
        let res3 = biblioteca.incrementar_stock_libro(1);

        assert!(res1.is_ok()); assert!(res2.is_ok()); assert!(res3.is_ok());

        assert_eq!(biblioteca.cantidad_de_copias_en_stock(5).unwrap(), 5, "ISBN 5 tiene 5 copias");
        assert_eq!(biblioteca.cantidad_de_copias_en_stock(3).unwrap(), 3, "ISBN 3 tiene 3 copias");
        assert_eq!(biblioteca.cantidad_de_copias_en_stock(1).unwrap(), 1, "ISBN 1 tiene 1 copias");

        // test 0s

        let res1 = biblioteca.decrementar_stock_libro(1);
        let res2 = biblioteca.decrementar_stock_libro(5);
        let res3 = biblioteca.decrementar_stock_libro(3);

        let res_dec = biblioteca.decrementar_stock_libro(1);

        let res_dec = match res_dec {
            Ok(_) => { panic!("Debería ser error") }
            Err(de) => {
                match de {
                    DoubleError::LocalError(err) => { err }
                    DoubleError::RemoteError(_) => { panic!("Deberia ser un error local") }
                }
            }
        };

        assert_eq!(res_dec, ErrorDecrementarStock::StockEsCero, "stock debería ser cero");

        // test overflow

        let res_inc = biblioteca.incrementar_stock_libro(u64::from(u32::MAX));

        let res_inc = match res_inc {
            Ok(_) => { panic!("Debería ser error") }
            Err(de) => {
                match de {
                    DoubleError::LocalError(err) => { err }
                    DoubleError::RemoteError(_) => { panic!("Deberia ser un error local") }
                }
            }
        };

        assert_eq!(res_inc, ErrorIncrementarStock::Overflow, "stock debería ser u32::MAX");
    }


    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_prestamos() {
        let mut biblioteca = Biblioteca {
            nombre: "asd".to_string(),
            direccion: "asd".to_string(),
            libros: BTreeMap::default(),
            clientes: BTreeMap::default()
        };

        // init realizar prestamos

        biblioteca.registrar_libro(libro_economia_1());
        biblioteca.registrar_libro(libro_xd_2());
        biblioteca.registrar_libro(libro_harrypotter_3());
        biblioteca.registrar_libro(libro_asd_4());
        biblioteca.registrar_libro(libro_estadistica_5());
        biblioteca.registrar_libro(libro_algo_u32max());

        match biblioteca.registrar_cliente(cliente_pepe()) {
            ResultRegistrarCliente::Exito { resultado_fm } => {
                match resultado_fm {
                    ResultSobreescribirArchivo::Success => {}
                    x => panic!("No deberían haber errores. {x:?}")
                }
            }
            ResultRegistrarCliente::ClienteYaExiste => { panic!("Cliente no existe") }
        }

        match biblioteca.registrar_cliente(cliente_manuel()) {
            ResultRegistrarCliente::Exito { resultado_fm } => {
                match resultado_fm {
                    ResultSobreescribirArchivo::Success => {}
                    x => panic!("No deberían haber errores. {x:?}")
                }
            }
            ResultRegistrarCliente::ClienteYaExiste => { panic!("Cliente no existe") }
        }

        let id_pepe = cliente_pepe().id;
        let id_manuel = cliente_manuel().id;

        let fecha5 = Fecha{ dia: 1, mes: 1, ano: 1 };
        let fecha3 = Fecha{ dia: 2, mes: 1, ano: 1 };

        let Ok(p5) = biblioteca.realizar_prestamo(id_pepe, 5, fecha5) else { panic!("Deberia ser exitoso") };
        let Ok(p3) = biblioteca.realizar_prestamo(id_manuel, 3, fecha3) else { panic!("Deberia ser exitoso") };

        let Some(cant_prestamos_pepe) = biblioteca.cantidad_prestamos_cliente(id_pepe) else { panic!() };
        let Some(cant_prestamos_manuel) = biblioteca.cantidad_prestamos_cliente(id_manuel) else { panic!() };

        assert_eq!(p5, cant_prestamos_pepe, "Debería ser equivalente");
        assert_eq!(p3, cant_prestamos_manuel, "Debería ser equivalente");

        // init cant copias

        let stock5 = biblioteca.cantidad_de_copias_en_stock(5);
        let stock3 = biblioteca.cantidad_de_copias_en_stock(3);

        // check

        assert!(!stock5.is_none(), "cantidad_de_copias_en_stock(): None");
        assert!(!stock3.is_none(), "cantidad_de_copias_en_stock(): None");

        assert_eq!(biblioteca.cantidad_de_copias_en_stock(5).unwrap(), 4, "Deberían haber 4 copias en stock");
        assert_eq!(biblioteca.cantidad_de_copias_en_stock(3).unwrap(), 2, "Deberían haber 2 copias en stock");

        // init prestamos por vencer

        let prestamos_a_vencer0 = biblioteca.prestamos_por_vencer(Fecha { dia: 1, mes: 1, ano: -1 }, 0);
        let prestamos_a_vencer1 = biblioteca.prestamos_por_vencer(fecha5.clone(), 0);
        let prestamos_a_vencer2 = biblioteca.prestamos_por_vencer(fecha5.clone(), 1);
        let prestamos_a_vencer2_2 = biblioteca.prestamos_por_vencer(Fecha { dia: 22, mes: 08, ano: 2002 }, 0);

        // check

        assert_eq!(prestamos_a_vencer0.len(), 0, "Deberíam haber 0 préstamos a vencer");
        assert_eq!(prestamos_a_vencer1.len(), 1, "Debería haber 1 préstamo a vencer");
        assert_eq!(prestamos_a_vencer2.len(), 2, "Deberían haber 2 préstamos a vencer");
        assert_eq!(prestamos_a_vencer2_2.len(), 2, "Deberían haber 0 préstamos a vencer");

        // init buscar prestamos

        let buscar_prestamo5 = biblioteca.buscar_prestamo(5, id_pepe);
        let buscar_prestamo3 = biblioteca.buscar_prestamo(3, id_manuel);

        // check

        assert!(!buscar_prestamo5.is_err(), "Error buscar_prestamo(): {:?}", buscar_prestamo5.unwrap_err());
        assert!(!buscar_prestamo3.is_err(), "Error buscar_prestamo(): {:?}", buscar_prestamo3.unwrap_err());

        assert_eq!(buscar_prestamo5.unwrap().isbn, 5, "El préstamo encontrado debería ser sobre el libro #5");
        assert_eq!(buscar_prestamo3.unwrap().isbn, 3, "El préstamo encontrado debería ser sobre el libro #3");

        // init-check devolver prestamos

        let devolucion_prestamo5 = biblioteca.devolver_libro(5, id_pepe, fecha5.clone());
        assert!(devolucion_prestamo5.is_ok(), "La devolucion no deberia dar error");

        let devolucion_prestamo3 = biblioteca.devolver_libro(3, id_manuel, fecha3.clone());
        assert!(devolucion_prestamo3.is_ok(), "La devolucion no deberia dar error");

        // init prestamos por vencer post-devolver

        let prestamos_a_vencer0 = biblioteca.prestamos_por_vencer(Fecha { dia: 1, mes: 1, ano: -1 }, 0);
        let prestamos_a_vencer1 = biblioteca.prestamos_por_vencer(fecha5, 0);
        let prestamos_a_vencer2 = biblioteca.prestamos_por_vencer(fecha5, 1);
        let prestamos_a_vencer2_2 = biblioteca.prestamos_por_vencer(Fecha { dia: 22, mes: 08, ano: 2002 }, 0);

        // check

        assert_eq!(prestamos_a_vencer0.len(), 0, "Deberíam haber 0 préstamos a vencer");
        assert_eq!(prestamos_a_vencer1.len(), 0, "Deberíam haber 0 préstamos a vencer");
        assert_eq!(prestamos_a_vencer2.len(), 0, "Deberíam haber 0 préstamos a vencer");
        assert_eq!(prestamos_a_vencer2_2.len(), 0, "Deberíam haber 0 préstamos a vencer");

        // init buscar prestamos post-devolver

        let buscar_prestamo5 = biblioteca.buscar_prestamo(5, id_pepe);
        let buscar_prestamo3 = biblioteca.buscar_prestamo(3, id_manuel);

        // check

        assert!(buscar_prestamo5.is_ok(), "Error buscar_prestamo(): {:?}", buscar_prestamo5.unwrap_err());
        assert!(buscar_prestamo3.is_ok(), "Error buscar_prestamo(): {:?}", buscar_prestamo3.unwrap_err());

        assert_eq!(buscar_prestamo5.clone().unwrap().isbn, 5, "El préstamo encontrado debería ser sobre el libro #5");
        assert_eq!(buscar_prestamo3.clone().unwrap().isbn, 3, "El préstamo encontrado debería ser sobre el libro #3");

        assert!(matches!(buscar_prestamo5.clone().unwrap().estado, EstadoPrestamo::Devuelto(_)), "El préstamo encontrado debería haber sido devuelto");
        assert!(matches!(buscar_prestamo3.clone().unwrap().estado, EstadoPrestamo::Devuelto(_)), "El préstamo encontrado debería haber sido devuelto");

        // init max prestamos (5)

        let Some(cant_stock_isbn5) = biblioteca.cantidad_stock_libro(5) else { panic!() };
        assert_eq!(cant_stock_isbn5, 5);

        let p1 = biblioteca.realizar_prestamo(id_pepe, 1, fecha5.clone());
        let p2 = biblioteca.realizar_prestamo(id_pepe, 2, fecha5.clone());
        let p3 = biblioteca.realizar_prestamo(id_pepe, 3, fecha5.clone());
        let p4 = biblioteca.realizar_prestamo(id_pepe, 4, fecha5.clone());
        let p5 = biblioteca.realizar_prestamo(id_pepe, 5, fecha5.clone());

        let p6 = biblioteca.realizar_prestamo(id_pepe, u64::from(u32::MAX), fecha3.clone());

        // check

        let Some(cant_prestamos_pepe) = biblioteca.cantidad_prestamos_cliente(id_pepe) else { panic!() };
        let Some(cant_prestamos_manuel) = biblioteca.cantidad_prestamos_cliente(id_manuel) else { panic!() };

        assert_eq!(cant_prestamos_pepe, 5);
        assert_eq!(cant_prestamos_manuel, 1);

        let Some(cant_stock_isbn5) = biblioteca.cantidad_stock_libro(5) else { panic!() };
        assert_eq!(cant_stock_isbn5, 4);

        assert!(p1.is_ok(), "El préstamo debería ser exitoso");
        assert!(p2.is_ok(), "El préstamo debería ser exitoso");
        assert!(p3.is_ok(), "El préstamo debería ser exitoso");
        assert!(p4.is_ok(), "El préstamo debería ser exitoso");
        assert!(p5.is_ok(), "El préstamo debería ser exitoso");

        assert!(p6.is_err(), "El préstamo no debería ser exitoso");
        let p6 = p6.unwrap_err();
        let p6 = match p6 {
            DoubleError::LocalError(err) => err,
            DoubleError::RemoteError(_) => panic!("El error deberia ser local")
        };

        assert_eq!(p6, ErrorRealizarPrestamo::PrestamosMaximosAlcanzados, "Debería haberse alcanzado el límite máximo de préstamos");

        // agotar stock

        let p1 = biblioteca.realizar_prestamo(id_manuel, 1, fecha5.clone());
        let Err(p1) = biblioteca.realizar_prestamo(id_manuel, 1, fecha5.clone()) else { panic!() };
        let DoubleError::LocalError(p1) = p1 else { panic!("El error debería ser local") };
        assert_eq!(p1, ErrorRealizarPrestamo::StockInsuficiente);

        let Err(p1) = biblioteca.realizar_prestamo(id_manuel, 1000, fecha5.clone()) else { panic!() };
        let DoubleError::LocalError(p1) = p1 else { panic!("El error debería ser local") };
        assert_eq!(p1, ErrorRealizarPrestamo::LibroNoExiste);

        // prestamo/cliente inexistentes: buscar_prestamo

        let p1 = biblioteca.buscar_prestamo(13548, 1);
        let p2 = biblioteca.buscar_prestamo(1, 13548);

        let Err(p1) = p1 else { panic!("Debe ser error") };
        let Err(p2) = p2 else { panic!("Debe ser error") };

        assert_eq!(p1, ErrorBuscarPrestamo::PrestamoInexistente);
        assert_eq!(p2, ErrorBuscarPrestamo::ClienteInexistente);

        // devolver_libro: prestamo/cliente inexistentes

        let p1 = biblioteca.devolver_libro(13548, 1, fecha5);
        let Err(p1) = p1 else { panic!("Debe ser error") };
        let DoubleError::LocalError(p1) = p1 else { panic!("El error debería ser local") };

        let p2 = biblioteca.devolver_libro(1, 13548, fecha5);
        let Err(p2) = p2 else { panic!("Debe ser error") };
        let DoubleError::LocalError(p2) = p2 else { panic!("El error debería ser local") };

        assert_eq!(p1, ErrorDevolverLibro::PrestamoInexistente);
        assert_eq!(p2, ErrorDevolverLibro::ClienteInexistente);

        // devolver_libro: libro ya devuelto

        let p1 = biblioteca.devolver_libro(1, 1, fecha5);
        if let Err(_) = p1 { panic!("No debe dar error.") }

        let p1 = biblioteca.devolver_libro(1, 1, fecha5);
        let Err(p1) = p1 else { panic!("No debe dar ok.") };
        let DoubleError::LocalError(p1) = p1 else { panic!("El error debería ser local") };

        assert_eq!(p1, ErrorDevolverLibro::LibroYaDevuelto);
    }

    #[test]
    fn test_prestamos_vencidos() {
        let mut biblioteca = Biblioteca {
            nombre: "asd".to_string(),
            direccion: "asd".to_string(),
            libros: BTreeMap::default(),
            clientes: BTreeMap::default()
        };

        biblioteca.registrar_cliente(cliente_manuel());
        let id_manuel = cliente_manuel().id;

        biblioteca.registrar_libro(libro_economia_1());

        let fecha_hoy = Fecha { dia: 2, mes: 1, ano: 0 };
        let fecha_ayer = Fecha { dia: 1, mes: 1, ano: 0 };

        match biblioteca.realizar_prestamo(id_manuel, 1, fecha_ayer) {
            Ok(res) => { assert_eq!(res, 1, "Debe haber solo un préstamo") }
            _ => { panic!("No debe haber error") }
        }

        let p_venc = biblioteca.prestamos_vencidos(fecha_hoy);
        assert_eq!(p_venc.len(), 1);

        let p_venc = biblioteca.prestamos_vencidos(fecha_ayer);
        assert_eq!(p_venc.len(), 0);
    }

    #[test]
    fn test_registrar_cliente() {
        let mut biblioteca = Biblioteca {
            nombre: "asd".to_string(),
            direccion: "asd".to_string(),
            libros: BTreeMap::default(),
            clientes: BTreeMap::default()
        };

        let r1 = biblioteca.registrar_cliente(cliente_pepe());
        let r2 = biblioteca.registrar_cliente(cliente_manuel());

        let r1 = match r1 {
            ResultRegistrarCliente::Exito { resultado_fm } => resultado_fm,
            ResultRegistrarCliente::ClienteYaExiste => panic!("El cliente no debería existir")
        };

        let r2 = match r2 {
            ResultRegistrarCliente::Exito { resultado_fm } => resultado_fm,
            ResultRegistrarCliente::ClienteYaExiste => panic!("El cliente no debería existir")
        };

        assert_eq!(r1, ResultSobreescribirArchivo::Success, "Debería ser exito");
        assert_eq!(r2, ResultSobreescribirArchivo::Success, "Debería ser exito");

        let r1 = biblioteca.registrar_cliente(cliente_pepe());
        let r2 = biblioteca.registrar_cliente(cliente_manuel());

        match r1 {
            ResultRegistrarCliente::Exito { .. } => panic!("Debería ser error"),
            ResultRegistrarCliente::ClienteYaExiste => ()
        }

        match r2 {
            ResultRegistrarCliente::Exito { .. } => panic!("Debería ser error"),
            ResultRegistrarCliente::ClienteYaExiste => ()
        }
    }

    #[test]
    fn test_doubleerror() {
        let double_error = DoubleError::LocalError(ErrorBuscarPrestamo::ClienteInexistente); // just an example

        assert!(double_error.is_local(), "Es un error local");

        // don't mind the error's content. it's being forced to create an example
        let double_error: DoubleError<ErrorBuscarPrestamo> = DoubleError::from(ResultSobreescribirArchivo::Success); // just an example

        assert!(double_error.is_remote(), "Es un error remoto");
    }
}

// Implemente un método con la siguiente firma dentro de la biblioteca:
//
// `get_historial_prestamos(&self, id_cliente: id_cliente, filtro_estado: estado) -> ???`
//
//
// Donde:
//
// id_cliente: identidicador del cliente del que se quieren consultar los préstamos.
// filtro_estado: parámetro que indica si se deben incluir todos los préstamos,
//      solo los “en préstamo” o solo los “devueltos”.
//
//
// El método debe devolver la colección de préstamos del cliente filtrados
//      según el estado indicado.
//
// Cada elemento del resultado debe incluir como mínimo:
//
// Título e ISBN del libro
// Fecha de vencimiento
// Fecha de devolución (si corresponde)
//
// Notas:
//
// Si el cliente no tiene préstamos que cumplan con el filtro, el método debe manejar
//      esa situación adecuadamente.
// El tipo de retorno y la colección utilizada quedan a criterio de la implementación.
// Incluir los tests necesarios que verifiquen el funcionamiento correcto
//      del método en todos los casos posibles.

pub struct InformacionPrestamoCliente {
    pub isbn: u64,
    pub titulo: String,
    pub fecha_vencimiento: Fecha,
    pub fecha_devolucion: Option<Fecha>
}

impl InformacionPrestamoCliente {
    pub fn from(libro: &Libro, prestamo: &Prestamo) -> Self {
        let fecha_devolucion = if let EstadoPrestamo::Devuelto(fecha) = prestamo.estado {
            Some(fecha)
        } else {
            None
        };

        Self {
            isbn: libro.isbn,
            titulo: libro.titulo.clone(),
            fecha_vencimiento: prestamo.vencimiento,
            fecha_devolucion,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FiltroEstadoLibro {
    Prestando, Devuelto, Ambos
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorObtenerHistorialPrestamos {
    ClienteInexistente,
    NoTienePrestamosQueCumplan,
}

// Implementación del entregable 2
impl Biblioteca {

    pub fn get_historial_prestamos(&self, id_cliente: u32, filtro_estado: FiltroEstadoLibro) -> Result<Vec<InformacionPrestamoCliente>, ErrorObtenerHistorialPrestamos> {
        let Some((_, prestamos)) = self.clientes.get(&id_cliente)
        else { return Err(ErrorObtenerHistorialPrestamos::ClienteInexistente); };

        let mut informacion_prestamos: Vec<InformacionPrestamoCliente> = Vec::new();

        let incluir_prestando: bool = filtro_estado == FiltroEstadoLibro::Prestando || filtro_estado == FiltroEstadoLibro::Ambos;
        let incluir_devuelto: bool = filtro_estado == FiltroEstadoLibro::Devuelto || filtro_estado == FiltroEstadoLibro::Ambos;

        for prestamo in prestamos {
            match prestamo.estado {
                EstadoPrestamo::Devuelto(_) => { if !incluir_devuelto { continue } }
                EstadoPrestamo::Prestando => { if !incluir_prestando { continue } }
            };

            // encontrar datos del libro
            let Some(libro) = self.libros.get(&prestamo.isbn)
            else { continue; }; // si el libro no existe, skipea y continua recorriendo

            informacion_prestamos.push(InformacionPrestamoCliente::from(libro, prestamo));
        }

        if informacion_prestamos.is_empty() {
            return Err(ErrorObtenerHistorialPrestamos::NoTienePrestamosQueCumplan);
        }

        Ok(informacion_prestamos)
    }

}

#[cfg(test)]
mod test_entregable_2 {
    use super::*;

    fn biblioteca_de_pepe() -> Biblioteca {
        Biblioteca::new(
            "biblio de pepe".to_string(),
            "donde queda".to_string(),
            Some(BTreeMap::from(
                [(1, libro_economia_1()),
                    (2, libro_xd_2()),
                    (3, libro_harrypotter_3()),
                    (4, libro_asd_4()),
                    (5, libro_estadistica_5()),
                    (u64::from(u32::MAX), libro_algo_u32max())])),
            None)
    }
    fn cliente_pepe() -> Cliente {
        Cliente::new(
            1,
            "pepe".to_string(),
            "123".to_string(),
            "pepe@gmail.com".to_string()
        )
    }
    fn cliente_manuel() -> Cliente {
        Cliente::new(
            3,
            "manuel".to_string(),
            "123".to_string(),
            "manuel@gmail.com".to_string()
        )
    }
    fn libro_economia_1() -> Libro {
        Libro::new(
            1,
            "Economía en una lección".to_string(),
            "xd".to_string(),
            1,
            Genero::Tecnico,
            1
        )
    }
    fn libro_xd_2() -> Libro {
        let mut libro = Libro::default();
        libro.isbn = 2;
        libro.titulo = "xd".to_string();
        libro.stock = 2;
        libro
    }
    fn libro_harrypotter_3() -> Libro {
        let mut libro = Libro::default();
        libro.isbn = 3;
        libro.titulo = "Harry Potter y qsy q mas".to_string();
        libro.stock = 3;
        libro
    }
    fn libro_asd_4() -> Libro {
        let mut libro = Libro::default();
        libro.isbn = 4;
        libro.titulo = "asd".to_string();
        libro.stock = 4;
        libro
    }
    fn libro_estadistica_5() -> Libro {
        let mut libro = Libro::default();
        libro.isbn = 5;
        libro.titulo = "Estadística".to_string();
        libro.stock = 5;
        libro
    }
    fn libro_algo_u32max() -> Libro {
        let mut libro = Libro::default();
        libro.isbn = u64::from(u32::MAX);
        libro.titulo = "algo".to_string();
        libro.stock = u32::MAX;
        libro
    }


    #[test]
    fn test_historial_prestamos() {
        let mut biblioteca = biblioteca_de_pepe();

        biblioteca.registrar_cliente(cliente_pepe());
        biblioteca.registrar_cliente(cliente_manuel());

        let fecha1 = Fecha { dia: 1, mes: 1, ano: 1 };
        let fecha2 = Fecha { dia: 10, mes: 1, ano: 1};

        biblioteca.realizar_prestamo(cliente_pepe().id, 1, fecha1);
        biblioteca.realizar_prestamo(cliente_pepe().id, 2, fecha1);

        // test prestandos (2)
        let res = biblioteca.get_historial_prestamos(cliente_pepe().id, FiltroEstadoLibro::Prestando);
        let Ok(res) = res else { panic!("Debería ser Ok"); };
        assert_eq!(res.len(), 2);

        // test ambos (2)
        let res = biblioteca.get_historial_prestamos(cliente_pepe().id, FiltroEstadoLibro::Ambos);
        let Ok(res) = res else { panic!("Debería ser Ok"); };
        assert_eq!(res.len(), 2);

        // test devueltos (0)
        let res = biblioteca.get_historial_prestamos(cliente_pepe().id, FiltroEstadoLibro::Devuelto);
        let Err(res) = res else { panic!("Debería ser Err"); };
        assert_eq!(res, ErrorObtenerHistorialPrestamos::NoTienePrestamosQueCumplan);

        // test cliente inexistente
        let res = biblioteca.get_historial_prestamos(13548, FiltroEstadoLibro::Devuelto);
        let Err(res) = res else { panic!("Debería ser Err"); };
        assert_eq!(res, ErrorObtenerHistorialPrestamos::ClienteInexistente);

        // devolver prestamo
        biblioteca.devolver_libro(1, cliente_pepe().id, fecha2);

        // test devueltos (1)
        let res = biblioteca.get_historial_prestamos(cliente_pepe().id, FiltroEstadoLibro::Devuelto);
        let Ok(res) = res else { panic!("Debería ser Ok"); };
        assert_eq!(res.len(), 1);
    }

}