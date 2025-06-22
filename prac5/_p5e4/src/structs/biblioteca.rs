use std::collections::BTreeMap;
use error_proc_macro::Error;
use serde::{Deserialize, Serialize};
use crate::structs::biblioteca_fm::BibliotecaFileManagement;
use crate::structs::cliente::Cliente;
use crate::structs::fecha::Fecha;
use crate::structs::prestamo::{EstadoPrestamo, Prestamo};
use super::libro::Libro;

const MAX_PRESTAMOS_ACTIVOS: usize = 5;

/// # Biblioteca
///
/// `nombre: String` - Nombre de la biblioteca<br>
/// `direccion: String` - Dirección física de la biblioteca<br>
/// `libros: BTreeMap<u64, Libro>` - Libros de la biblioteca.<br>
/// `prestamos: BTreeMap<u32, (Cliente, Vec<Prestamo>)>` - <b> BTreeMap<ID del cliente, (Cliente, Vec<Prestamo>)>
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, PartialOrd, Debug)]
pub struct Biblioteca {
    pub nombre: String,
    pub direccion: String,
    pub libros: BTreeMap<u64, Libro>,
    pub clientes: BTreeMap<u32, (Cliente, Vec<Prestamo>)> // <ID cliente, (Cliente, Vec<Préstamo>)>
}

#[derive(Error, Clone, PartialEq, PartialOrd)]
pub enum ErrorDecrementarStock {
    StockEsCero, LibroNoExiste
}

#[derive(Error, Clone, PartialEq, PartialOrd)]
pub enum ErrorIncrementarStock {
    LibroNoExiste, Overflow
}

#[derive(Error, Clone, PartialEq, PartialOrd)]
pub enum ErrorRealizarPrestamo {
    PrestamosMaximosAlcanzados, StockInsuficiente, ClienteInexistente, LibroNoExiste
}

#[derive(Error, Clone, PartialEq, PartialOrd)]
pub enum ErrorBuscarPrestamo {
    PrestamoInexistente, ClienteInexistente
}

#[derive(Error, Clone, PartialEq, PartialOrd)]
pub enum ErrorDevolverLibro {
    PrestamoInexistente, ClienteInexistente, LibroYaDevuelto
}

type Libros = BTreeMap<u64, Libro>;
type Clientes = BTreeMap<u32, (Cliente, Vec<Prestamo>)>;

impl Biblioteca {

    /// ### fn new() -> Biblioteca
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
            biblioteca.libros = biblioteca.leer_archivo_libros().unwrap_or_default()
        };

        if let Some(data) = clientes {
            biblioteca.clientes = data;
            biblioteca.sobreescribir_archivo_clientes();
        } else {
            biblioteca.clientes = biblioteca.leer_archivo_clientes().unwrap_or_default()
        };

        biblioteca
    }

    /// ### fn cantidad_de_copias_en_stock(isbn) -> Option<u32>
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

    /// ### fn decrementar_stock_libro(isbn) -> Result<u32, ErrorDecrementarStock>
    /// Devuelve la cantidad de libros en stock después de decrementarla en 1
    ///
    /// #### Recibe:<br>
    /// `isbn` - ID del libro a consultar
    ///
    /// #### Devuelve:<br>
    /// `u32` - Cantidad de libros después de decrementar<br>
    /// `ErrorDecrementarStock` - El stock es cero o el libro no existe
    pub fn decrementar_stock_libro(&mut self, isbn: u64) -> Result<u32, ErrorDecrementarStock> {
        let stock_restante = match self.libros.get_mut(&isbn) {
            None => return Err(ErrorDecrementarStock::LibroNoExiste),
            Some(libro) => {
                if libro.stock == 0 {
                    return Err(ErrorDecrementarStock::StockEsCero)
                }
                libro.stock-= 1;
                libro.stock
            }
        };

        self.sobreescribir_archivo_libros();
        Ok(stock_restante)
    }


    /// ### fn incrementar_stock_libro(isbn) -> Result<u32, ErrorIncrementarStock>
    /// Devuelve la cantidad de libros en stock después de incrementarla en 1
    ///
    /// #### Recibe:<br>
    /// `isbn` - ID del libro a consultar
    ///
    /// #### Devuelve:<br>
    /// `u32` - Cantidad de libros después de decrementar<br>
    /// `ErrorIncrementarStock` - El stock es `u32::MAX` o el libro no existe
    pub fn incrementar_stock_libro(&mut self, isbn: u64) -> Result<u32, ErrorIncrementarStock> {
        let stock_restante = match self.libros.get_mut(&isbn) {
            Some(libro) => {
                if libro.stock == u32::MAX {
                    return Err(ErrorIncrementarStock::Overflow)
                }
                libro.stock-= 1;
                libro.stock
            },
            None => return Err(ErrorIncrementarStock::LibroNoExiste)
        };

        self.sobreescribir_archivo_libros();
        Ok(stock_restante)
    }

    /// ### fn cantidad_prestamos_cliente(cliente) -> Option<usize>
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

    /// ### fn cantidad_stock_libro(isbn) -> Option<u32>
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

    /// ### fn realizar_prestamo(cliente, isbn, vencimiento) -> Result(usize, ErrorRealizarPrestamo)
    /// Realiza un préstamo del libro en nombre del cliente con el vencimiento especificado
    ///
    /// #### Recibe:<br>
    /// `cliente` - Cliente a efectuar el préstamo<br>
    /// `isbn` - ID del libro a prestar<br>
    /// `vencimiento` - Fecha de vencimiento del préstamo<br>
    ///
    /// #### Devuelve:<br>
    /// `usize` - Cantidad de préstamos del cliente, incluyendo el recién realizado
    pub fn realizar_prestamo(&mut self, id_cliente: u32, isbn: u64, vencimiento: Fecha) -> Result<usize, ErrorRealizarPrestamo> /* <Cant. préstamos vigentes del cliente, Error> */ {
        match self.libros.get(&isbn) {
            Some(libro) => {
                if libro.stock == 0 {
                    return Err(ErrorRealizarPrestamo::StockInsuficiente)
                }
            },
            None => return Err(ErrorRealizarPrestamo::LibroNoExiste)
        }

        // obtener cliente
        let datos_cliente = match self.clientes.get_mut(&id_cliente) {
            Some(dato) => {
                dato
            },
            None => { return Err(ErrorRealizarPrestamo::ClienteInexistente) }
        };

        // check cant. max. prestamos
        let cant_libros_no_devueltos = datos_cliente.1.iter().filter(|p| p.estado == EstadoPrestamo::Prestando).count();
        if cant_libros_no_devueltos >= MAX_PRESTAMOS_ACTIVOS {
            return Err(ErrorRealizarPrestamo::PrestamosMaximosAlcanzados);
        }

        // si el préstamo alguna vez se realizó: eliminarlo para reemplazarlo.
        datos_cliente.1.retain(|p| p.isbn != isbn);

        // realizar préstamo
        let prestamo = Prestamo::new(isbn, id_cliente, vencimiento, None, EstadoPrestamo::Prestando);
        datos_cliente.1.push(prestamo);

        // reducir stock
        if let Some(libro) = self.libros.get_mut(&isbn) {
            libro.stock-= 1;
        }

        self.sobreescribir_archivo_clientes();

        Ok(cant_libros_no_devueltos + 1)
    }

    /// ### fn prestamos_a_vencer(feca_hoy, dias) -> Vec<&Prestamo>
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

        let mut fecha_limte = fecha_hoy;
        fecha_limte.sumar_dias(dias);
        let fecha_limite = fecha_limte; // quitar mutabilidad

        for prestamos_cliente in self.clientes.values() {
            for prestamo in &prestamos_cliente.1 {

                match &prestamo.devolucion {
                    Some(_) => continue, // ya fue devuelto, no contabilizar
                    None => {
                        if prestamo.devolucion.is_none() && prestamo.estado == EstadoPrestamo::Prestando && prestamo.vencimiento <= fecha_limite {
                            prestamos_por_vencer.push(prestamo);
                        }
                    }
                }

            }
        }

        prestamos_por_vencer
    }

    /// ### fn prestamos_vencidos(fecha_hoy) -> Vec<&Prestamo>
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

    /// ### fn buscar_prestamo(isbn, id_cliente) -> Result<&Prestamo, ErrorBuscarPrestamo>
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

    /// ### fn devolver_libro(isbn, id_cliente, fecha_hoy) -> Result<&Prestamo, ErrorDevolverLibro>
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
    pub fn devolver_libro(&mut self, isbn: u64, id_cliente: u32, fecha_hoy: Fecha) -> Result<u32, ErrorDevolverLibro> {
        let mut data_cliente = match self.clientes.get_mut(&id_cliente) {
            Some(dato) => { dato },
            None => return Err(ErrorDevolverLibro::ClienteInexistente),
        }; todo!("algo no entendí de la mutabilidaad");
        
        let mut prestamo = 
            if let Some(data) = data_cliente.1.iter_mut().find(|prestamo| prestamo.isbn == isbn ) {
                data
            } else { return Err(ErrorDevolverLibro::PrestamoInexistente) };

        if prestamo.estado == EstadoPrestamo::Devuelto {
            return Err(ErrorDevolverLibro::LibroYaDevuelto)
        }

        if prestamo.devolucion.is_none() && prestamo.estado == EstadoPrestamo::Devuelto {
            return Err(ErrorDevolverLibro::LibroYaDevuelto)
        }

        prestamo.devolucion = Some(fecha_hoy);
        prestamo.estado = EstadoPrestamo::Devuelto;

        let stock_libro = if let Some(libro) = self.libros.get_mut(&isbn) {
            libro.stock+= 1;
            libro.stock
        } else { 0 };
        
        self.sobreescribir_archivo_libros();
        self.sobreescribir_archivo_clientes();

        Ok(stock_libro)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use crate::structs::biblioteca::{Biblioteca, ErrorBuscarPrestamo, ErrorDecrementarStock, ErrorDevolverLibro, ErrorIncrementarStock, ErrorRealizarPrestamo};
    use crate::structs::cliente::Cliente;
    use crate::structs::fecha::Fecha;
    use crate::structs::libro::{Genero, Libro};
    use crate::structs::prestamo::{EstadoPrestamo, Prestamo};

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
                    (u32::MAX as u64, libro_algo_u32max())])),
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
        libro.isbn = u32::MAX as u64;
        libro.titulo = "algo".to_string();
        libro.stock = u32::MAX;
        libro
    }

    #[test]
    fn test_cant_copias() {
        let mut biblioteca = biblioteca_de_pepe();

        // test dec

        let res = biblioteca.decrementar_stock_libro(5000);

        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ErrorDecrementarStock::LibroNoExiste);

        let res = biblioteca.incrementar_stock_libro(5000);

        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), ErrorIncrementarStock::LibroNoExiste);

        //

        assert_eq!(biblioteca.cantidad_de_copias_en_stock(5).unwrap(), 5, "ISBN 5 tiene 5 copias");
        assert_eq!(biblioteca.cantidad_de_copias_en_stock(3).unwrap(), 3, "ISBN 3 tiene 3 copias");
        assert_eq!(biblioteca.cantidad_de_copias_en_stock(1).unwrap(), 1, "ISBN 1 tiene 1 copias");

        biblioteca.decrementar_stock_libro(5);
        biblioteca.decrementar_stock_libro(3);
        biblioteca.decrementar_stock_libro(1);

        assert_eq!(biblioteca.cantidad_de_copias_en_stock(5).unwrap(), 4, "ISBN 5 tiene 4 copias");
        assert_eq!(biblioteca.cantidad_de_copias_en_stock(3).unwrap(), 2, "ISBN 3 tiene 2 copias");
        assert_eq!(biblioteca.cantidad_de_copias_en_stock(1).unwrap(), 0, "ISBN 1 tiene 0 copias");

        // test inc

        biblioteca.incrementar_stock_libro(5);
        biblioteca.incrementar_stock_libro(3);
        biblioteca.incrementar_stock_libro(1);

        assert_eq!(biblioteca.cantidad_de_copias_en_stock(5).unwrap(), 5, "ISBN 5 tiene 5 copias");
        assert_eq!(biblioteca.cantidad_de_copias_en_stock(3).unwrap(), 3, "ISBN 3 tiene 3 copias");
        assert_eq!(biblioteca.cantidad_de_copias_en_stock(1).unwrap(), 1, "ISBN 1 tiene 1 copias");

        // test 0

        biblioteca.decrementar_stock_libro(1);

        biblioteca.decrementar_stock_libro(5);
        biblioteca.decrementar_stock_libro(3);
        let dec = biblioteca.decrementar_stock_libro(1);

        assert_eq!(dec.unwrap_err(), ErrorDecrementarStock::StockEsCero, "stock debería ser cero");

        // test overflow

        let inc = biblioteca.incrementar_stock_libro(u32::MAX as u64);
        assert_eq!(inc.unwrap_err(), ErrorIncrementarStock::Overflow, "stock debería ser u32::MAX");
    }

    #[test]
    fn test_prestamos() {
        let mut biblioteca = biblioteca_de_pepe();

        // init realizar prestamos

        let fecha5 = Fecha{ dia: 1, mes: 1, ano: 1 };
        let fecha3 = Fecha{ dia: 2, mes: 1, ano: 1 };

        let p5 = biblioteca.realizar_prestamo(cliente_pepe(), 5, fecha5.clone());
        let p3 = biblioteca.realizar_prestamo(cliente_manuel(), 3, fecha3.clone());

        let cant_prestamos_pepe = if let Some(val) = biblioteca.cantidad_prestamos_cliente(cliente_pepe().id) { val } else { panic!() };
        let cant_prestamos_manuel = if let Some(val) = biblioteca.cantidad_prestamos_cliente(cliente_manuel().id) { val } else { panic!() };

        assert_eq!(cant_prestamos_pepe, 1);
        assert_eq!(cant_prestamos_manuel, 1);

        // check

        if p5.is_err() { panic!("Error inesperado realizando préstamo: {:?}", p5.unwrap_err()) }
        if p3.is_err() { panic!("Error inesperado realizando préstamo: {:?}", p3.unwrap_err()) }

        assert_eq!(p5.unwrap(), 1, "Debería tener 1 préstamo");
        assert_eq!(p3.unwrap(), 1, "Debería tener 1 préstamo");

        // init cant copias

        let stock5 = biblioteca.cantidad_de_copias_en_stock(5);
        let stock3 = biblioteca.cantidad_de_copias_en_stock(3);

        // check

        if stock5.is_none() { panic!("cantidad_de_copias_en_strock(): None") }
        if stock3.is_none() { panic!("cantidad_de_copias_en_strock(): None") }

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

        let buscar_prestamo5 = biblioteca.buscar_prestamo(5, cliente_pepe().id);
        let buscar_prestamo3 = biblioteca.buscar_prestamo(3, cliente_manuel().id);

        // check

        if buscar_prestamo5.is_err() { panic!("Error buscar_prestamo(): {:?}", buscar_prestamo5.unwrap_err()) }
        if buscar_prestamo3.is_err() { panic!("Error buscar_prestamo(): {:?}", buscar_prestamo3.unwrap_err()) }

        assert_eq!(buscar_prestamo5.unwrap().isbn, 5, "El préstamo encontrado debería ser sobre el libro #5");
        assert_eq!(buscar_prestamo3.unwrap().isbn, 3, "El préstamo encontrado debería ser sobre el libro #3");

        // init-check devolver prestamos

        let devolucion_prestamo5 = biblioteca.devolver_libro(5, cliente_pepe().id, fecha5.clone());
        if devolucion_prestamo5.is_err() { panic!("") }

        let devolucion_prestamo3 = biblioteca.devolver_libro(3, cliente_manuel().id, fecha3.clone());
        if devolucion_prestamo3.is_err() { panic!("") }

        // init prestamos por vencer post-devolver

        let prestamos_a_vencer0 = biblioteca.prestamos_por_vencer(Fecha { dia: 1, mes: 1, ano: -1 }, 0);
        let prestamos_a_vencer1 = biblioteca.prestamos_por_vencer(fecha5.clone(), 0);
        let prestamos_a_vencer2 = biblioteca.prestamos_por_vencer(fecha5.clone(), 1);
        let prestamos_a_vencer2_2 = biblioteca.prestamos_por_vencer(Fecha { dia: 22, mes: 08, ano: 2002 }, 0);

        // check

        assert_eq!(prestamos_a_vencer0.len(), 0, "Deberíam haber 0 préstamos a vencer");
        assert_eq!(prestamos_a_vencer1.len(), 0, "Deberíam haber 0 préstamos a vencer");
        assert_eq!(prestamos_a_vencer2.len(), 0, "Deberíam haber 0 préstamos a vencer");
        assert_eq!(prestamos_a_vencer2_2.len(), 0, "Deberíam haber 0 préstamos a vencer");

        // init buscar prestamos post-devolver

        let buscar_prestamo5 = biblioteca.buscar_prestamo(5, cliente_pepe().id);
        let buscar_prestamo3 = biblioteca.buscar_prestamo(3, cliente_manuel().id);

        // check

        if buscar_prestamo5.is_err() { panic!("Error buscar_prestamo(): {:?}", buscar_prestamo5.unwrap_err()) }
        if buscar_prestamo3.is_err() { panic!("Error buscar_prestamo(): {:?}", buscar_prestamo3.unwrap_err()) }

        assert_eq!(buscar_prestamo5.clone().unwrap().isbn, 5, "El préstamo encontrado debería ser sobre el libro #5");
        assert_eq!(buscar_prestamo3.clone().unwrap().isbn, 3, "El préstamo encontrado debería ser sobre el libro #3");

        assert_eq!(buscar_prestamo5.clone().unwrap().estado, EstadoPrestamo::Devuelto, "El préstamo encontrado debería haber sido devuelto");
        assert_eq!(buscar_prestamo3.clone().unwrap().estado, EstadoPrestamo::Devuelto, "El préstamo encontrado debería haber sido devuelto");

        assert!(buscar_prestamo5.unwrap().devolucion.is_some(), "El préstamo encontrado debería haber sido devuelto");
        assert!(buscar_prestamo3.unwrap().devolucion.is_some(), "El préstamo encontrado debería haber sido devuelto");

        // init max prestamos (5)

        let cant_stock_isbn5 = if let Some(val) = biblioteca.cantidad_stock_libro(5) { val } else { panic!() };
        assert_eq!(cant_stock_isbn5, 5);

        let p1 = biblioteca.realizar_prestamo(cliente_pepe(), 1, fecha5.clone());
        let p2 = biblioteca.realizar_prestamo(cliente_pepe(), 2, fecha5.clone());
        let p3 = biblioteca.realizar_prestamo(cliente_pepe(), 3, fecha5.clone());
        let p4 = biblioteca.realizar_prestamo(cliente_pepe(), 4, fecha5.clone());
        let p5 = biblioteca.realizar_prestamo(cliente_pepe(), 5, fecha5.clone());

        let p6 = biblioteca.realizar_prestamo(cliente_pepe(), u32::MAX as u64, fecha3.clone());

        // check

        let cant_prestamos_pepe = if let Some(val) = biblioteca.cantidad_prestamos_cliente(cliente_pepe().id) { val } else { panic!() };
        let cant_prestamos_manuel = if let Some(val) = biblioteca.cantidad_prestamos_cliente(cliente_manuel().id) { val } else { panic!() };

        assert_eq!(cant_prestamos_pepe, 5);
        assert_eq!(cant_prestamos_manuel, 1);

        let cant_stock_isbn5 = if let Some(val) = biblioteca.cantidad_stock_libro(5) { val } else { panic!() };
        assert_eq!(cant_stock_isbn5, 4);

        assert!(p1.is_ok(), "El préstamo debería ser exitoso");
        assert!(p2.is_ok(), "El préstamo debería ser exitoso");
        assert!(p3.is_ok(), "El préstamo debería ser exitoso");
        assert!(p4.is_ok(), "El préstamo debería ser exitoso");
        assert!(p5.is_ok(), "El préstamo debería ser exitoso");

        assert!(p6.is_err(), "El préstamo no debería ser exitoso");
        assert_eq!(p6.unwrap_err(), ErrorRealizarPrestamo::PrestamosMaximosAlcanzados, "Debería haberse alcanzado el límite máximo de préstamos");

        // agotar stock

        let p1 = biblioteca.realizar_prestamo(cliente_manuel(), 1, fecha5.clone());
        let p1 = if let Err(err) = biblioteca.realizar_prestamo(cliente_manuel(), 1, fecha5.clone()) { err } else { panic!() };
        assert_eq!(p1, ErrorRealizarPrestamo::StockInsuficiente);

        let p1 = if let Err(err) = biblioteca.realizar_prestamo(cliente_manuel(), 1000, fecha5.clone()) { err } else { panic!() };
        assert_eq!(p1, ErrorRealizarPrestamo::LibroNoExiste);

        // prestamo/cliente inexistentes: buscar_prestamo

        let p1 = biblioteca.buscar_prestamo(13548, 1);
        let p2 = biblioteca.buscar_prestamo(1, 13548);

        let p1 = if let Err(error) = p1 { error } else { panic!("Debe ser error") };
        let p2 = if let Err(error) = p2 { error } else { panic!("Debe ser error") };

        assert_eq!(p1, ErrorBuscarPrestamo::PrestamoInexistente);
        assert_eq!(p2, ErrorBuscarPrestamo::ClienteInexistente);

        // devolver_libro: prestamo/cliente inexistentes

        let p1 = biblioteca.devolver_libro(13548, 1, fecha5);
        let p1 = if let Err(error) = p1 { error } else { panic!("Debe ser error") };

        let p2 = biblioteca.devolver_libro(1, 13548, fecha5);
        let p2 = if let Err(error) = p2 { error } else { panic!("Debe ser error") };

        assert_eq!(p1, ErrorDevolverLibro::PrestamoInexistente);
        assert_eq!(p2, ErrorDevolverLibro::ClienteInexistente);

        // devolver_libro: libro ya devuelto

        let p1 = biblioteca.devolver_libro(1, 1, fecha5);
        if let Err(_) = p1 { panic!("No debe dar error.") };

        let p1 = biblioteca.devolver_libro(1, 1, fecha5);
        let p1 = if let Err(err) = p1 { err } else { panic!("No debe dar ok.") };

        assert_eq!(p1, ErrorDevolverLibro::LibroYaDevuelto);
    }

    #[test]
    fn test_prestamos_vencidos() {
        let mut biblioteca = biblioteca_de_pepe();

        let fecha_hoy = Fecha { dia: 2, mes: 1, ano: 0 };
        let fecha_ayer = Fecha { dia: 1, mes: 1, ano: 0 };

        match biblioteca.realizar_prestamo(cliente_manuel(), 1, fecha_ayer) {
            Ok(res) => { assert_eq!(res, 1, "Debe haber solo un préstamo") }
            Err(_) => { panic!("No debe haber error") }
        }

        let p_venc = biblioteca.prestamos_vencidos(fecha_hoy);
        assert_eq!(p_venc.len(), 1);

        let p_venc = biblioteca.prestamos_vencidos(fecha_ayer);
        assert_eq!(p_venc.len(), 0);
    }

}