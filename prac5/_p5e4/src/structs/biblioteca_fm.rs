// biblioteca file management

use std::{fs, io};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use error_proc_macro::Error;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{error, Error, Value};
use crate::structs::biblioteca::Biblioteca;
use crate::structs::cliente::Cliente;
use crate::structs::fecha::Fecha;
use crate::structs::libro::{Genero, Libro};
use crate::structs::prestamo::{EstadoPrestamo, Prestamo};

type Libros = BTreeMap<u64, Libro>;
type Clientes = BTreeMap<u32, (Cliente, Vec<Prestamo>)>;

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

#[derive(Debug)]
pub enum ResultSobreescribirArchivo {
    Success,
    IOError(io::Error), //io::Error == ()
    SerializationError(error::Error),
}

#[derive(Error)]
pub enum ErrorLeerArchivo {
    IOError(io::Error),
    DeserializationError(error::Error),
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
        Err(error) => { return ResultSobreescribirArchivo::SerializationError(error) }
    };

    match fs::write(file_abs_path, text) {
        Err(error) => ResultSobreescribirArchivo::IOError(error),
        _ => ResultSobreescribirArchivo::Success
    }
}

fn leer_archivo(filepath: String) -> Result<Value, ErrorLeerArchivo> {
    let mut file = match File::open(filepath) {
        Ok(file) => { file }
        Err(error) => { return Err(ErrorLeerArchivo::IOError(error)) }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(error) => return Err(ErrorLeerArchivo::IOError(error))
    }

    match serde_json::from_str(&contents) {
        Err(error) => Err(ErrorLeerArchivo::DeserializationError(error)),
        Ok(value) => Ok(value),
    }
}

fn leer_archivo_parsed<T>(value: Value) -> Result<T, ErrorLeerArchivo> where T: DeserializeOwned {
    let data: T = match serde_json::from_value::<T>(value) {
        Ok(data) => { data }
        Err(error) => { return Err(ErrorLeerArchivo::DeserializationError(error)) }
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
mod tests {
    use crate::structs::biblioteca_fm::{archivo_filepath, leer_archivo, leer_archivo_parsed, sobreescribir_archivo, BibliotecaFileManagement, Clientes, DataBiblioteca, ErrorLeerArchivo, Libros, ResultSobreescribirArchivo, BASE_FOLDER};
    use crate::structs::cliente::Cliente;
    use crate::structs::fecha::Fecha;
    use crate::structs::libro::{Genero, Libro};
    use crate::structs::prestamo::{EstadoPrestamo, Prestamo};
    use crate::structs::biblioteca::Biblioteca;

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
        let io_error = ResultSobreescribirArchivo::IOError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "test error"
        ));
        
        assert_eq!(success1, success2);
        assert_ne!(success1, io_error);
        
        // Test ErrorLeerArchivo equality
        let io_error1 = ErrorLeerArchivo::IOError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "test error"
        ));
        let io_error2 = ErrorLeerArchivo::IOError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "different message but same type"
        ));
        
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