// biblioteca file management

use std::{fs, io};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use error_proc_macro::Error;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{error, Value};
use crate::structs::streamingrust::StreamingRust;
use crate::structs::usuario::Usuario;

type Usuarios = BTreeMap<u64, Usuario>;

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

#[derive(Error)]
pub enum ErrorSobreescribirArchivo {
    IOError(io::Error), //io::Error == ()
    SerializationError(error::Error),
}

#[derive(Error)]
pub enum ErrorLeerArchivo {
    IOError(io::Error),
    DeserializationError(error::Error),
}

impl PartialEq for ErrorSobreescribirArchivo {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl PartialEq for ErrorLeerArchivo {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
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

    let text = match json_string {
        Ok(text) => { text }
        Err(error) => { return Err(ErrorSobreescribirArchivo::SerializationError(error)); }
    };

    match fs::write(file_abs_path, text) {
        Err(error) => Err(ErrorSobreescribirArchivo::IOError(error)),
        _ => Ok(())
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
mod tests {
    use crate::structs::streamingrust_fm::{archivo_filepath, leer_archivo, leer_archivo_parsed, ErrorLeerArchivo, StreamingRustFileManagement, BASE_FOLDER, DEFAULT_FILE_NAME};
    use std::collections::HashMap;
    use crate::structs::fecha::Fecha;
    use crate::structs::streamingrust::StreamingRust;
    use crate::structs::streamingrust_fm::{sobreescribir_archivo, ErrorSobreescribirArchivo, Usuarios};
    use crate::structs::suscripcion::{F64Precio, MedioDePago, Suscripcion, TipoSuscripcion};
    use crate::structs::usuario::Usuario;

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
        let io_error1 = ErrorSobreescribirArchivo::IOError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "test error"
        ));
        let io_error2 = ErrorSobreescribirArchivo::IOError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "test error 2"
        ));
        
        assert_eq!(io_error1, io_error2);

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