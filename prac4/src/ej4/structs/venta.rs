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

use std::collections::HashMap;
use crate::structs::fecha::Fecha;
use crate::structs::producto::{CategoriaProducto, Producto};
use crate::structs::vendedor_cliente::{Cliente, Vendedor};

const DESCUENTO_SUSCRIPCION_NEWSLETTER: f32 = 5.0;

pub enum MedioDePago {
    Credito, Debito, Transferencia, Efectivo
}

// El sistema debe permitir registrar las ventas realizadas y asociar el medio de pago utilizado.
// Los medios de pago aceptados son:
//     tarjeta de crédito,
//     tarjeta de débito,
//     transferencia bancaria
//     y efectivo.

pub struct Venta {
    pub fecha: Fecha,
    pub cliente: u32, // cliente.dni
    pub vendedor: u32, // vendedor.legajo
    pub medio_de_pago: MedioDePago,
    pub productos: HashMap<Producto, u16>,
}

impl Venta {

    // ➢ Crear una venta con: fecha, cliente, vendedor, medio de pago y un listado de productos con sus cantidades.
    fn new(fecha: Fecha, cliente: u32, vendedor: u32, medio_de_pago: MedioDePago, productos: HashMap<Producto, u16>) -> Option<Venta> {
        if !fecha.es_fecha_valida() { return None }

        Some(Venta {
            fecha, cliente, vendedor, medio_de_pago, productos
        })
    }

    // ➢ Calcular el precio final de una venta en base a los productos que hay en ella.
    //     Para calcularlo tenga en cuenta que pueden haber determinados productos de alguna categoría
    //         donde debería aplicarse un descuento.
    //     Tanto la categoría como el porcentaje de descuento a aplicar son datos que le brinda el sistema.
    //     Es decir el sistema tiene una lista de las categorías con el descuento a aplicar.
    //     Además se debe aplicar un porcentaje de descuento general si el cliente tiene suscripción al newsletter.

    fn precio_final(&self, descuentos_porc: &HashMap<CategoriaProducto, f32>, suscrito_newsletter: bool) -> Option<f32> {
        for val in descuentos_porc.values() {
            if *val < 0.0 || *val > 100.0 { return None }
        }

        let mut precio_final = 0.0;
        // calcular precios y sumarlos
        for (producto, cant) in &self.productos {
            if let Some(descuento) = descuentos_porc.get(&producto.categoria) {
                precio_final+= producto.precio * (1.0 - *descuento/100.0) * (*cant as f32)
            } else {
                precio_final+= producto.precio * (*cant as f32)
            }
        }

        if suscrito_newsletter { precio_final*= 1.0 - DESCUENTO_SUSCRIPCION_NEWSLETTER/100.0 }

        Some(precio_final)
    }
}