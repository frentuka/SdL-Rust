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

//
// fecha.rs
//

// epoch: 01/01/1970 00:00:00.0000 (dd/mm/yyyy hh:mm:ss.millis)
pub struct Fecha {
    // anteriormente usaba datos para dia, mes, ano (por haberlo importado de ej3/prac3)
    // pero al necesitar ordenar prefiero usar el sistema clasico de EPOCH
    // y añadir luego funciones para calcular dia, mes, año si es necesario
    pub millis_since_epoch: u128,
}

//
// producto.rs
//

#[derive(Hash, Copy, Clone, PartialEq, Eq)]
pub enum CategoriaProducto {
    Cosmetico, Tecnologia, HomeDeco, Almacen, Ferreteria, Drogueria, Textil
}

pub struct Producto {
    pub nombre: String,
    pub apellido: String,
    pub categoria: CategoriaProducto,
    pub precio: f32,
}

//
// vendedor_cliente.rs
//

pub struct Vendedor {
    pub legajo: u32,
    pub antiguedad_anos: u8,
    pub salario: f64,
}

//     De ellos se conoce
//         nombre,
//         apellido,
//         dirección,
//         dni
pub struct Cliente {
    pub nombre: String,
    pub apellido: String,
    pub direccion: String,
    pub dni: u32 // u32::MAX = 4.xxx.xxx.xxx
}

impl Vendedor {
    fn new(legajo: u32, antiguedad_anos: u8, salario: f64) -> Option<Vendedor> {
        if salario < 0.0 { return None }

        Some(Vendedor {
            legajo,
            antiguedad_anos,
            salario
        })
    }
}

impl Cliente {
    fn new(nombre: &str, apellido: &str, direccion: &str, dni: u32) -> Cliente {
        Cliente {
            nombre: nombre.to_string(),
            apellido: apellido.to_string(),
            direccion: direccion.to_string(),
            dni
        }
    }
}

//
// venta.rs
//

const DESCUENTO_SUSCRIPCION_NEWSLETTER: f32 = 5.0;

pub enum MedioDePago {
    Credito, Debito, Transferencia, Efectivo
}

pub struct Venta {
    pub fecha: Fecha,
    pub cliente: u32, // cliente.dni
    pub vendedor: u32, // vendedor.legajo
    pub medio_de_pago: MedioDePago,
    pub productos: HashMap<Producto, u16>,
}

impl Venta {

    // ➢ Crear una venta con: fecha, cliente, vendedor, medio de pago y un listado de productos con sus cantidades.
    fn new(fecha: Fecha, cliente: u32, vendedor: u32, medio_de_pago: MedioDePago, productos: HashMap<Producto, u16>) -> Venta {
        Venta {
            fecha, cliente, vendedor, medio_de_pago, productos
        }
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

//
// comercio.rs
//

struct Comercio {
    vendedores: HashMap<u32, Vendedor>,
    // modelo. cada vez que se ejecute una venta debería sobreescribirse los datos del cliente
    // propósito: poder a futuro obtener datos del cliente desde una venta
    // sin tener que copiar la totalidad.
    clientes: HashMap<u32, Cliente>,
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

//
// entregable #2
//

pub struct InformeVentaIndividual {
    venta: Venta, // fecha, productos, sus cantidades y medio de pago incluido en Venta
    cant_total_productos: u16, // No especifica ser necesario pero por si acaso.
    monto_total: f64, // obtenible desde la función venta.precio_final() pero no incluido en venta
}

pub struct HistorialVentas {
    vendeor: u32,
    // en caso de que no contenga ventas que cumplan la condición
    // preferiría devolver un vec vacío, pero ya que el enunciado pide especificarlo
    // devolveré un Option<Vec> aunque me parezca redundante a un vec que isEmpty.
    // ¿Qué otro problema podría causar que ventas.isEmpty() == true?
    ventas: Option<Vec<InformeVentaIndividual>>,
}

impl Comercio {

    // 🧾 Implementar una funcionalidad que permita obtener un informe de ventas realizadas por un vendedor específico, filtrando solo aquellas ventas que contengan al menos un producto de una categoría dada.
    //
    // Este informe debe incluir, ordenado cronológicamente de la venta más reciente a la más antigua, lo siguiente para cada venta:
    //
    // -Fecha de la venta
    // -Productos vendidos y sus cantidades
    // -Monto total final de la venta
    // -Medio de pago utilizado
    //
    // La consulta se debe realizar a partir de un identificador único del vendedor
    //      (por ejemplo, su número de legajo, según cómo lo hayan modelado),
    //      y una categoría de producto como filtro.

    // En caso de que el vendedor no tenga ventas que cumplan esa condición,
    //      el sistema debe reflejar esa situación de forma adecuada.
    //
    // 🔧 Esta funcionalidad debe implementarse como un métod.o
    //      dentro del struct principal del sistema.
    //
    // 🧪 Además, deben incluir los tests necesarios para verificar el correcto funcionamiento
    //      de esta funcionalidad.

    // Firma esperada del métod.o:
    // get_historial_ventas(id: id_vendedor, categoria: CategoriaProducto) -> ???

    fn get_historial_ventas(&self, vendedor: u32, categoria: CategoriaProducto) -> Option<HistorialVentas> {
        if !self.vendedores.contains_key(&vendedor) {
            return None;
        }

        let mut historial_ventas = HistorialVentas {
            vendeor: vendedor,
            ventas: Some(vec![]),
        };

        for venta in &self.ventas {
            if venta.vendedor == vendedor {
                let mut debe_procesarse = false;

                for (producto, cant) in &venta.productos {
                    if producto.categoria == categoria {
                        debe_procesarse = true;
                    }
                }

                if debe_procesarse {
                    // crear informe de la venta e incluirlo en el historial
                    // insertar de forma ordenada para evitar ordenar en el futuro
                }
            }
        }

        // let Some(ventas) = historial_ventas.ventas else {
        //     // si isEmpty, convertir a None
        // };

        todo!()
    }

}