mod structs;

/// # Ej. 10
///
/// ##### Para una biblioteca se desea implementar un sistema de préstamos de libros.
/// De la biblioteca se conoce:
///     el nombre,
///     la dirección,
///     las copias de los libros a disposición para prestar
///     y los préstamos efectuados.
/// <br><br>
/// Los "libros a disposición" es un registro donde se indica la cantidad de ejemplares
/// que tiene a disposición para prestar de determinado libro.
/// De cada libro se conoce:
///     el título,
///     autor,
///     número de páginas,
///     género (novela, infantil, técnico, otros).
/// <br><br>
/// Para registrar un préstamo se requiere:
///     el libro,
///     el cliente,
///     la fecha de vencimiento del préstamo,
///     la fecha de devolución
///     y el estado (devuelto o en préstamo)
/// <br><br>
/// Del cliente se conoce:
///     el nombre,
///     teléfono
///     y dirección de correo electrónico.
///
/// #### Implemente los métodos necesarios para realizar las siguientes acciones:
///
/// <p>➔ obtener cantidad de copias:
///     dado un determinado libro retorna el retorna la cantidad de copias a disposición
///     que hay para prestar de dicho libro.</p>
///
/// <p>➔ decrementar cantidad de copias a disposición:
///     dado un libro decrementa en 1 la cantidad de copias de libros a disposición para prestar.</p>
///
/// <p>➔ incrementar cantidad de copias a disposición:
///     dado un libro incremente en 1 la cantidad de copias del libro a disposición para ser prestado.</p>
///
/// <p>➔ contar préstamos de un cliente:
///     devuelve la cantidad de préstamos en estado “en préstamo” de un determinado cliente.</p>
///
/// <p>➔ ver la cantidad disponible de un determinado libro:
///     retorna la cantidad de libros disponibles del registro de “copias a disposición” de un determinado libro.</p>
///
/// <p>➔ realizar un préstamo de un libro para un cliente:
///     crea un préstamo de un libro para un determinado cliente cumpliendo con lo siguiente
///         ◆ el cliente no tenga más de 5 préstamos en el estado “en préstamo”
///         ◆ haya al menos una copia disponible en el registro de copias a disposición.
///             De ser así descuenta 1 en el registro de “copias a disposición” y retorna true,
///                 si no cumple con alguna de las condiciones retorna false.</p>
///
/// <p>➔ ver préstamos a vencer el los próximos días:
///     retorna una lista de préstamos a vencer el los próximos días, el valor de días es pasado por parámetro.</p>
///
/// <p>➔ ver los préstamos vencidos:
///     retorna una lista de préstamos en el estado “en préstamos”
///     donde la fecha de vencimiento es menor a la fecha actual.</p>
///
/// <p>➔ buscar préstamo:
///     dado un libro y un cliente busca un préstamo y lo retorna si existe.</p>
///
/// <p>➔ devolver libro:
///     dado un libro y un cliente se busca el préstamo y se cambia al estado “devuelto”,
///     se registra la fecha de devolución y se incrementa la cantidad de libros en 1
///     del libro devuelto en el registro de copias a disposición.</p>
fn main() {


}