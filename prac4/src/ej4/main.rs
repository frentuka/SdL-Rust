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

use std::collections::{BTreeMap, HashMap};

//
// fecha.rs
//

// epoch: 01/01/1970 00:00:00.0000 (dd/mm/yyyy hh:mm:ss.millis)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fecha {
    // anteriormente usaba datos para dia, mes, ano (por haberlo importado de ej3/prac3)
    // pero al necesitar ordenar prefiero usar el sistema clasico de EPOCH
    // y añadir luego funciones para calcular dia, mes, año si es necesario
    pub millis_since_epoch: u128,
}

//
// producto.rs
//

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum CategoriaProducto {
    Cosmetico, Tecnologia, HomeDeco, Almacen, Ferreteria, Drogueria, Textil
}

#[derive(Debug, Clone, PartialEq)]
pub struct Producto {
    pub nombre: String,
    pub categoria: CategoriaProducto,
    pub precio: f32,
}

//
// vendedor_cliente.rs
//

#[derive(Debug, Clone, PartialEq)]
pub struct Vendedor {
    pub legajo: u32,
    pub antiguedad_anos: u8,
    pub salario: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cliente {
    pub dni: u32, // u32::MAX = 4.xxx.xxx.xxx
    pub nombre: String,
    pub apellido: String,
    pub direccion: String,
    pub suscrito_newsletter: bool,
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
    fn new(nombre: &str, apellido: &str, direccion: &str, dni: u32, suscrito_newsletter: bool) -> Cliente {
        Cliente {
            dni,
            nombre: nombre.to_string(),
            apellido: apellido.to_string(),
            direccion: direccion.to_string(),
            suscrito_newsletter,
        }
    }
}

//
// venta.rs
//

const DESCUENTO_SUSCRIPCION_NEWSLETTER: f32 = 5.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MedioDePago {
    Credito, Debito, Transferencia, Efectivo
}

#[derive(Debug, Clone, PartialEq)]
pub struct Venta {
    pub fecha: Fecha,
    pub cliente: u32, // cliente.dni
    pub vendedor: u32, // vendedor.legajo
    pub medio_de_pago: MedioDePago,
    pub productos: Vec<(Producto, u16)>
}

impl Venta {
    // ➢ Crear una venta con: fecha, cliente, vendedor, medio de pago y un listado de productos con sus cantidades.
    fn new(fecha: Fecha, cliente: u32, vendedor: u32, medio_de_pago: MedioDePago, productos: Vec<(Producto, u16)>) -> Venta {
        Venta {
            fecha, cliente, vendedor, medio_de_pago, productos
        }
    }

    // Solía ser Option<f32> porque un descuento podría ser inválido.
    // En cambio, si un descuento es invalido, se desestima.
    fn precio_final(&self, descuentos_porc: &HashMap<CategoriaProducto, f32>, suscrito_newsletter: bool) -> f32 {
        let mut precio_final = 0.0;
        // calcular precios y sumarlos
        for (producto, cant) in &self.productos {
            let descuento = if let Some(descuento) = descuentos_porc.get(&producto.categoria) {
                if *descuento >= 0.0 && *descuento <= 100.0 { *descuento }
                else { 0.0 }
            } else { 0.0 };

            // aplicar
            precio_final+= producto.precio * (1.0 - descuento/100.0) * (*cant as f32)
        }

        if suscrito_newsletter { precio_final*= 1.0 - DESCUENTO_SUSCRIPCION_NEWSLETTER/100.0 }

        precio_final
    }
}

//
// comercio.rs
//

struct Comercio {
    vendedores: HashMap<u32, Vendedor>,
    clientes: BTreeMap<u32, Cliente>,
    descuentos: HashMap<CategoriaProducto, f32>,
    ventas: Vec<(Venta, f32)>
}

#[derive(Default)]
struct ReporteTotal {
    reporte_categorias: HashMap<CategoriaProducto, u32>,
    reporte_vendedores: HashMap<u32, u32> // <legajo, ventas>
}

#[derive(Debug, Clone, PartialEq)]
enum ErrorRegistrarVenta {
    VendedorInexistente{ legajo_vendedor: u32 },
    ErrorDesconocido(String)
}

impl Comercio {
    // Registra una venta en el vector de ventas, calculando su precio final antes de insertarla.
    // El cliente es opcional:
    //      Si se brinda None, debe existir en el map de clientes. Si no existe, no se aplicará el descuento.
    //      Si se brinda Some, se reemplazará en el map de clientes. Si existe y corresponde, se aplicará el descuento.
    // Como ninguna parte del enunciado exige lo contrario, se permitirá el anonimato del cliente.
    fn registrar_venta(&mut self, venta: Venta, cliente: Option<Cliente>) -> Result<&(Venta, f32), ErrorRegistrarVenta> {
        if let None = self.vendedores.get(&venta.vendedor) {
            return Err(ErrorRegistrarVenta::VendedorInexistente { legajo_vendedor: venta.vendedor });
        }

        let suscrito_newsletter = if let Some(cliente) = cliente {
            let suscrito = cliente.suscrito_newsletter;
            self.clientes.insert(cliente.dni, cliente); // insertar o reemplazar con datos actualizados
            suscrito
        } else if let Some(cliente) = self.clientes.get(&venta.cliente) {
            cliente.suscrito_newsletter
        } else { false };

        let precio_final = venta.precio_final(&self.descuentos, suscrito_newsletter);

        self.ventas.push((venta, precio_final));

        if let Some(venta) = self.ventas.last() {
            Ok(venta)
        } else {
            Err(ErrorRegistrarVenta::ErrorDesconocido("El elemento ingresado al vector no se pudo recuperar".to_string()))
        }
    }

    // ➢ Para llevar un control de ventas realizadas se debe implementar
    //      un reporte que permita visualizar las ventas totales por categoría de producto y otro por vendedor.
    fn generar_reporte_total(&self) -> ReporteTotal {
        let mut reporte_total = ReporteTotal::default();

        for (venta, _) in &self.ventas {
            for (producto, cant) in &venta.productos {
                *reporte_total.reporte_categorias.entry(producto.categoria).or_insert(0)+= u32::from(*cant);
                *reporte_total.reporte_vendedores.entry(venta.vendedor).or_insert(0)+= u32::from(*cant);
            }
        }

        reporte_total
    }
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vendedor() {
        let vendedor_some = Vendedor::new(10, 5, 170000.0);
        let vendedor_none = Vendedor::new(10, 5, -170000.0);

        assert!(vendedor_some.is_some(), "Debería devolver un vendedor");
        assert!(vendedor_none.is_none(), "No debería devolver un vendedor, su salario es negativo");
    }

    #[test]
    fn test_new_cliente() {
        let cliente = Cliente {
            dni: 0,
            nombre: "Mario".to_string(),
            apellido: "Santos".to_string(),
            direccion: "Avenida Siempre Viva 742".to_string(),
            suscrito_newsletter: false,
        };

        let cliente_new = Cliente::new(
            "Mario", "Santos", "Avenida Siempre Viva 742", 0, false
        );

        assert_eq!(cliente, cliente_new, "Ambos clientes deben ser idénticos");
    }

    #[test]
    fn test_new_venta() {
        let productos: Vec<(Producto, u16)> = vec![
            (Producto { nombre: "asd1".to_string(), categoria: CategoriaProducto::Cosmetico, precio: 15.0, }, 5),
            (Producto { nombre: "asd2".to_string(), categoria: CategoriaProducto::HomeDeco, precio: 10.0, }, 10),
        ];

        let venta = Venta {
            fecha: Fecha { millis_since_epoch: 1 },
            cliente: 1,
            vendedor: 2,
            medio_de_pago: MedioDePago::Credito,
            productos: productos.clone(),
        };

        let venta_new = Venta::new(
            Fecha { millis_since_epoch: 1 }, 1, 2, MedioDePago::Credito, productos
        );

        assert_eq!(venta, venta_new, "Ambas ventas deben ser idénticas");
    }

    #[test]
    fn test_precio_final() {
        let productos: Vec<(Producto, u16)> = vec![
            (Producto { nombre: "asd1".to_string(), categoria: CategoriaProducto::Cosmetico, precio: 15.0, }, 5),
            (Producto { nombre: "asd2".to_string(), categoria: CategoriaProducto::HomeDeco, precio: 10.0, }, 10),
        ];

        let venta = Venta {
            fecha: Fecha { millis_since_epoch: 1 },
            cliente: 1,
            vendedor: 2,
            medio_de_pago: MedioDePago::Credito,
            productos: productos.clone(),
        };

        let descuentos = HashMap::from([(CategoriaProducto::HomeDeco, 10.0)]);

        assert_eq!(venta.precio_final(&descuentos, true), (15.0 * 5.0 + 9.0 * 10.0) * (100.0 - DESCUENTO_SUSCRIPCION_NEWSLETTER) / 100.0);
    }

    #[test]
    fn test_registrar_venta() {
        let descuentos = HashMap::from([(CategoriaProducto::HomeDeco, 10.0)]);
        let mut comercio = Comercio {
            vendedores: Default::default(),
            clientes: BTreeMap::from([(1, Cliente {
                dni: 1,
                nombre: "asd".to_string(),
                apellido: "asd".to_string(),
                direccion: "asd".to_string(),
                suscrito_newsletter: true,
            })]),
            descuentos: descuentos.clone(),
            ventas: vec![],
        };

        let productos: Vec<(Producto, u16)> = vec![
            (Producto { nombre: "asd1".to_string(), categoria: CategoriaProducto::Cosmetico, precio: 15.0, }, 5),
            (Producto { nombre: "asd2".to_string(), categoria: CategoriaProducto::HomeDeco, precio: 10.0, }, 10),
        ];

        let venta = Venta {
            fecha: Fecha { millis_since_epoch: 1 },
            cliente: 1,
            vendedor: 2,
            medio_de_pago: MedioDePago::Credito,
            productos,
        };

        let result = comercio.registrar_venta(venta.clone(), None);
        assert_eq!(result, Err(ErrorRegistrarVenta::VendedorInexistente { legajo_vendedor: 2 }));

        //
        // valido
        //

        let mut comercio = Comercio {
            vendedores: HashMap::from([(2, Vendedor {
                legajo: 2,
                antiguedad_anos: 10,
                salario: 300.0,
            })]),
            clientes: BTreeMap::from([(1, Cliente {
                dni: 1,
                nombre: "asd".to_string(),
                apellido: "asd".to_string(),
                direccion: "asd".to_string(),
                suscrito_newsletter: true,
            })]),
            descuentos,
            ventas: vec![],
        };

        let result = comercio.registrar_venta(venta.clone(), None);
        assert_eq!(result, Ok(&(venta, (15.0 * 5.0 + 9.0 * 10.0) * (100.0 - DESCUENTO_SUSCRIPCION_NEWSLETTER) / 100.0)));
    }

    #[test]
    fn test_generar_reporte_total() {
        let descuentos = HashMap::from([(CategoriaProducto::HomeDeco, 10.0)]);

        let productos: Vec<(Producto, u16)> = vec![
            (Producto { nombre: "asd1".to_string(), categoria: CategoriaProducto::Cosmetico, precio: 15.0, }, 5),
            (Producto { nombre: "asd2".to_string(), categoria: CategoriaProducto::HomeDeco, precio: 10.0, }, 10),
        ];

        let venta_vend1 = Venta {
            fecha: Fecha { millis_since_epoch: 1 },
            cliente: 0,
            vendedor: 1,
            medio_de_pago: MedioDePago::Credito,
            productos: productos.clone(),
        };

        let venta_vend2 = Venta {
            fecha: Fecha { millis_since_epoch: 1 },
            cliente: 0,
            vendedor: 2,
            medio_de_pago: MedioDePago::Credito,
            productos,
        };

        let precio_ventas = (15.0 * 5.0 + 9.0 * 10.0) * (100.0 - DESCUENTO_SUSCRIPCION_NEWSLETTER) / 100.0;

        let comercio = Comercio {
            vendedores: Default::default(),
            clientes: BTreeMap::from([(1, Cliente {
                dni: 1,
                nombre: "asd".to_string(),
                apellido: "asd".to_string(),
                direccion: "asd".to_string(),
                suscrito_newsletter: true,
            })]),
            descuentos: descuentos.clone(),
            ventas: vec![
                (venta_vend1, precio_ventas),
                (venta_vend2, precio_ventas)
            ]
        };

        let reporte = comercio.generar_reporte_total();

        let Some(dato_hd) = reporte.reporte_categorias.get(&CategoriaProducto::HomeDeco) else { panic!("Debería existir") };
        let Some(dato_cm) = reporte.reporte_categorias.get(&CategoriaProducto::Cosmetico) else { panic!("Debería existir") };

        assert_eq!(*dato_hd, 20, "Deberian haber 20 productos de HomeDeco");
        assert_eq!(*dato_cm, 10, "Deberian haber 10 productos de Comsetico");

        let Some(dato_v) = reporte.reporte_vendedores.get(&2) else { panic!("Debería existir") };
        assert_eq!(*dato_v, 15, "Debería tener 15 productos vendidos en total");
    }
}