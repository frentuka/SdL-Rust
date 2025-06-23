// biblioteca file management

use std::{fs, io};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use error_proc_macro::Error;
use serde::de::DeserializeOwned;
use serde_json::{error, Error, Value};
use crate::structs::biblioteca::Biblioteca;
use crate::structs::cliente::Cliente;
use crate::structs::libro::Libro;
use crate::structs::prestamo::Prestamo;

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

const BASE_FOLDER: &str = "R:/appcrap/RustRover/SdL-Rust/prac5/_p5e4/res/";
const FILE_NAME_FORMAT: &str = "{}_{}.json"; // {biblioteca.nombre}_{libros/prestamos}.json
const LIBROS_FILENAME: &str = "libros";
const CLIENTES_FILENAME: &str = "clientes";
fn archivo_filepath(library_name: &str, file_kind: &str) -> String {
    format!("{BASE_FOLDER}{library_name}_{file_kind}.json")
}

//
// IO part
//

fn sobreescribir_archivo(filename: String, data: &DataBiblioteca) -> ResultSobreescribirArchivo {
    let json_string = match data {
        DataBiblioteca::Libros(data) => { serde_json::to_string_pretty(data) },
        DataBiblioteca::Clientes(data) => { serde_json::to_string_pretty(data) }
    };

    let text = match json_string {
        Ok(text) => { text }
        Err(error) => { return ResultSobreescribirArchivo::SerializationError(error) }
    };

    match fs::write(filename, text) {
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
// Implementation
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