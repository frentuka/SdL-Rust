use std::collections::BTreeMap;
use crate::structs::cliente::Cliente;
use crate::structs::fecha::Fecha;
use crate::structs::prestamo::{EstadoPrestamo, Prestamo};
use super::libro::Libro;

/// # Biblioteca
/// 
/// `nombre: String` - Nombre de la biblioteca<br>
/// `direccion: String` - Dirección física de la biblioteca<br>
/// `libros: BTreeMap<u64, Libro>` - Libros de la biblioteca<br>
/// `prestamos: BTreeMap<u32, (Cliente, Vec<Prestamo>)>` - <b> BTreeMap<ID del cliente, (Cliente, Vec<Prestamo>)>
#[derive(Default, Clone, PartialEq, PartialOrd, Debug)]
pub struct Biblioteca {
    pub nombre: String,
    pub direccion: String,
    pub libros: BTreeMap<u64, Libro>,
    pub prestamos: BTreeMap<u32, (Cliente, Vec<Prestamo>)> // <ID cliente, Préstamos>
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
enum ErrorDecrementarStock {
    StockEsCero, LibroNoExiste
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
enum ErrorIncrementarStock {
    LibroNoExiste, Overflow
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
enum ErrorRealizarPrestamo {
    PrestamosMaximosAlcanzados, StockInsuficiente, LibroNoExiste
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
enum ErrorBuscarPrestamo {
    PrestamoInexistente, ClienteInexistente
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
enum ErrorDevolverLibro {
    PrestamoInexistente, ClienteInexistente, LibroYaDevuelto
}

impl Biblioteca {

    /// ### fn new() -> Biblioteca
    /// Crea una nueva instancia de biblioteca
    ///
    /// #### Recibe:<br>
    /// `nombre` - Nombre de la biblioteca<br>
    /// `direccion` - Dirección de la biblioteca<br>
    /// `libros` - Opcional: Lista de libros de la biblioteca<br>
    /// `prestamos` - Opcional: Lista de préstamos de la biblioteca<br>
    ///
    /// #### Devuelve:
    /// `Biblioteca` - Nueva instancia de Biblioteca
    fn new(nombre: String, direccion: String, libros: Option<BTreeMap<u64, Libro>>, prestamos: Option<BTreeMap<u32, (Cliente, Vec<Prestamo>)>>) -> Biblioteca {
        Biblioteca {
            nombre,
            direccion,
            libros: libros.unwrap_or_default(),
            prestamos: prestamos.unwrap_or_default()
        }
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
    fn cantidad_de_copias_en_stock(&self, isbn: u64) -> Option<u32> {
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
    fn decrementar_stock_libro(&mut self, isbn: u64) -> Result<u32, ErrorDecrementarStock> {
        match self.libros.get_mut(&isbn) {
            Some(libro) => {
                if libro.stock == 0 {
                    Err(ErrorDecrementarStock::StockEsCero)
                } else {
                    libro.stock-= 1;
                    Ok(libro.stock)
                }
            },
            None => Err(ErrorDecrementarStock::LibroNoExiste)
        }
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
    fn incrementar_stock_libro(&mut self, isbn: u64) -> Result<u32, ErrorIncrementarStock> {
        match self.libros.get_mut(&isbn) {
            Some(libro) => {
                if libro.stock == u32::MAX {
                    Err(ErrorIncrementarStock::Overflow)
                } else {
                    libro.stock+= 1;
                    Ok(libro.stock)
                }
            },
            None => Err(ErrorIncrementarStock::LibroNoExiste)
        }
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
    fn cantidad_prestamos_cliente(&self, cliente: u32) -> Option<usize> {
        // match self.prestamos.get(cliente) {
        //     Some(cliente) => {
        //         Some(cliente.len())
        //     },
        //     None => None
        // }

        self.prestamos.get(&cliente).map(|cliente| cliente.1.len()) // compiler suggestion
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
    fn cantidad_stock_libro(&self, isbn: u64) -> Option<u32> {
        self.libros.get(&isbn).map(|libro| libro.stock) // compiler suggestion
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
    fn realizar_prestamo(&mut self, cliente: Cliente, isbn: u64, vencimiento: Fecha) -> Result<usize, ErrorRealizarPrestamo> /* <Cant. préstamos del cliente, Error> */ {
        match self.libros.get(&isbn) {
            Some(libro) => {
                if libro.stock == 0 {
                    return Err(ErrorRealizarPrestamo::StockInsuficiente)
                }
            },
            None => return Err(ErrorRealizarPrestamo::LibroNoExiste)
        }

        let prestamo = Prestamo::new(isbn, cliente.id, vencimiento, None, EstadoPrestamo::Prestando);

        match self.prestamos.get_mut(&cliente.id) {
            Some(dato) => {
                let cant_libros_no_devueltos = dato.1.iter().filter(|p| p.devolucion.is_none() && p.estado == EstadoPrestamo::Prestando).count();
                if cant_libros_no_devueltos >= 5 {
                    return Err(ErrorRealizarPrestamo::PrestamosMaximosAlcanzados);
                }

                // si el préstamo alguna vez se realizó, eliminar el antiguo préstamo
                dato.1.retain(|p| p.isbn != isbn);

                // quitar stock al libro a prestar
                // no puedo usar self.decrementar_stock_libro() porque tendría 2 borrows mutables en simultáneo
                if let Some(libro) = self.libros.get_mut(&isbn) { libro.stock-= 1 } // compiler suggestion

                dato.1.push(prestamo);
                Ok(cant_libros_no_devueltos)
            },
            None => {
                // insertar cliente
                self.prestamos.insert(cliente.id, (cliente, vec![prestamo]));
                if let Some(libro) = self.libros.get_mut(&isbn) { libro.stock-= 1 } // compiler suggestion
                Ok(1)
            }
        }
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
    fn prestamos_por_vencer(&self, fecha_hoy: Fecha, dias: u32) -> Vec<&Prestamo> {
        let mut prestamos_por_vencer: Vec<&Prestamo> = Vec::new();

        let mut fecha_limte = fecha_hoy;
        fecha_limte.sumar_dias(dias);
        let fecha_limite = fecha_limte; // quitar mutabilidad

        for prestamos_cliente in self.prestamos.values() {
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
    fn prestamos_vencidos(&self, fecha_hoy: Fecha) -> Vec<&Prestamo> {
        let mut prestamos_vencidos: Vec<&Prestamo> = Vec::new();

        for prestamos_cliente in self.prestamos.values() {
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
    fn buscar_prestamo(&self, isbn: u64, id_cliente: u32) -> Result<&Prestamo, ErrorBuscarPrestamo> {
        match self.prestamos.get(&id_cliente) {
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
    /// `&Prestamo` - El préstamo del libro que se ha devuelto<br>
    /// `ErrorDevolverLibro` - El cliente o el préstamo no existen o ya fue devuelto
    fn devolver_libro(&mut self, isbn: u64, id_cliente: u32, fecha_hoy: Fecha) -> Result<&Prestamo, ErrorDevolverLibro> {
        match self.prestamos.get_mut(&id_cliente) {
            Some(dato) => {
                for prestamo in dato.1.iter_mut() {
                    if prestamo.isbn == isbn {
                        if prestamo.estado == EstadoPrestamo::Devuelto {
                            return Err(ErrorDevolverLibro::LibroYaDevuelto)
                        }

                        prestamo.devolucion = Some(fecha_hoy);
                        prestamo.estado = EstadoPrestamo::Devuelto;

                        // no puedo usar self.incrementar_stock_libro() porque tendría 2 borrows mutables en simultáneo
                        if let Some(libro) = self.libros.get_mut(&isbn) { libro.stock-= 1 }

                        return Ok(prestamo)
                    }
                }
                Err(ErrorDevolverLibro::PrestamoInexistente)
            },
            None => Err(ErrorDevolverLibro::ClienteInexistente)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use crate::structs::biblioteca::{Biblioteca, ErrorDecrementarStock, ErrorIncrementarStock};
    use crate::structs::cliente::Cliente;
    use crate::structs::fecha::Fecha;
    use crate::structs::libro::Libro;
    use crate::structs::prestamo::EstadoPrestamo;

    fn biblioteca_de_pepe() -> Biblioteca {
        Biblioteca::new(
            "biblio de pepe".to_string(),
            "donde queda".to_string(),
            Some(BTreeMap::from(
                [(1, libro_economia_1()),
                    (3, libro_harrypotter_3()),
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
        let mut libro = Libro::default();
        libro.isbn = 1;
        libro.titulo = "Economía en una lección".to_string();
        libro.stock = 1;
        libro
    }
    fn libro_harrypotter_3() -> Libro {
        let mut libro = Libro::default();
        libro.isbn = 3;
        libro.titulo = "Harry Potter y qsy q mas".to_string();
        libro.stock = 3;
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

        assert_eq!(buscar_prestamo5.unwrap().estado, EstadoPrestamo::Devuelto, "El préstamo encontrado debería haber sido devuelto");
        assert_eq!(buscar_prestamo3.unwrap().estado, EstadoPrestamo::Devuelto, "El préstamo encontrado debería haber sido devuelto");
    }
}