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
use crate::structs::producto::CategoriaProducto;
use crate::structs::vendedor_cliente::{Cliente, Vendedor};
use crate::structs::venta::Venta;

struct Comercio {
    vendedores: HashMap<u32, Vendedor>,
    clientes: HashMap<u32, Cliente>, // modelo. cada vez que se ejecute una venta, debería sobreescribirse los datos del cliente
    descuentos: HashMap<CategoriaProducto, f32>,
    ventas: Vec<Venta>
}

#[derive(Default)]
struct ReporteTotal {
    reporte_categorias: HashMap<CategoriaProducto, u16>,
    reporte_vendedores: HashMap<u32, u16> // <legajo, ventas>
}

impl Comercio {

    // ➢ Para llevar un control de ventas realizadas se debe implementar
    //      un reporte que permita visualizar las ventas totales por categoría de producto y otro por vendedor.
    
    fn generar_reporte_total(&self) -> ReporteTotal {
        let mut reporte_total = ReporteTotal::default();
        
        for venta in &self.ventas {
            for (producto, cant) in &venta.productos {
                *reporte_total.reporte_categorias.entry(producto.categoria).or_insert(0)+= cant;
                *reporte_total.reporte_vendedores.entry(venta.vendedor).or_insert(0)+= cant;
            }
        }
        
        reporte_total
    }
    
}