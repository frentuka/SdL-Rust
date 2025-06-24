/*

4 - Se requiere implementar un sistema de ventas de productos.
    De cada producto se conoce
        el nombre,
        una categoría
        y un precio base,
        y algunos productos pueden tener descuentos aplicables dependiendo de la categoría.
    Además, se debe registrar al vendedor que realizó la venta y al cliente.
    De ellos se conoce
        nombre,
        apellido,
        dirección,
        dni
    y del vendedor
        nro de legajo,
        antigüedad
        y salario.
    Los clientes pueden tener un beneficio de descuento si tienen suscripción al newsletter,
    de ser así se tiene el correo electrónico del mismo.

El sistema debe permitir registrar las ventas realizadas y asociar el medio de pago utilizado.
Los medios de pago aceptados son:
    tarjeta de crédito,
    tarjeta de débito,
    transferencia bancaria
    y efectivo.

Implemente las estructuras, funciones asociadas y traits necesarios para resolver las siguientes acciones:

➢ Crear una venta con: fecha, cliente, vendedor, medio de pago y un listado de productos con sus cantidades.

➢ Calcular el precio final de una venta en base a los productos que hay en ella.
    Para calcularlo tenga en cuenta que pueden haber determinados productos de alguna categoría
        donde debería aplicarse un descuento.
    Tanto la categoría como el porcentaje de descuento a aplicar son datos que le brinda el sistema.
    Es decir el sistema tiene una lista de las categorías con el descuento a aplicar.
    Además se debe aplicar un porcentaje de descuento general si el cliente tiene suscripción al newsletter.

➢ Para llevar un control de ventas realizadas se debe implementar un reporte que permita visualizar las ventas totales
    por categoría de producto y otro por vendedor.

 */

//     y del vendedor
//         nro de legajo,
//         antigüedad
//         y salario.
pub struct Vendedor {
    pub legajo: u16,
    pub antiguedad_anos: u8,
    pub salario: f64,
}

//     De ellos se conoce
//         nombre,
//         apellido,
//         dirección,
//         dni
pub struct Cliente<'a> {
    pub nombre: &'a str,
    pub apellido: &'a str,
    pub direccion: &'a str,
    pub dni: u32 // u32::MAX = 4.xxx.xxx.xxx
}

impl Vendedor {
    fn new(legajo: u16, antiguedad_anos: u8, salario: f64) -> Option<Vendedor> {
        if salario < 0.0 { return None }
        
        Some(Vendedor {
            legajo, antiguedad_anos, salario
        })
    }
}

impl<'a> Cliente<'a> {
    fn new(nombre: &'a str, apellido: &'a str, direccion: &'a str, dni: u32) -> Cliente<'a> {
        Cliente {
            nombre, apellido, direccion, dni
        }
    }
}