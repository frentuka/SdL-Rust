/*

5- La empresa XYZ es una plataforma de intercambio de criptoactivos
    que permite a los usuarios comprar y vender distintas criptomonedas.

    La plataforma permite el registro de usuarios y la gestión de sus balances
        en distintas criptomonedas y en dinero fíat.
    De los usuarios se conoce:
        nombre, apellido, email, dni, y si está validada su identidad o no.

    Cada usuario tiene un balance de las criptomonedas que se ofrecen en la plataforma.
    De las criptomonedas se conoce:
        nombre, prefijo y un listado de blockchains donde se pueden enviar o recibir.
    De cada blockchain se conoce el nombre, prefijo.

Implemente las estructuras, funciones asociadas y traits necesarios
    para resolver las siguientes acciones relacionadas al usuario:

➢ Ingresar dinero: se recibe un monto en fiat de un usuario
    y se acredita al balance de fiat de dicho usuario. Además se crea una transacción del hecho donde los datos que se guardan son:fecha, tipo(ingreso de dinero), monto, usuario.

➢ Comprar determinada criptomoneda: dado un monto de fiat se compra una cantidad de determinada criptomoneda,
    tenga en cuenta que al momento de realizar la operación se obtiene del sistema la cotización actual de la criptomoneda para acreditar la correspondiente proporción en el balance de la cripto y desacreditar en el balance de fiat. Luego de ello se registra la transacción con los siguientes datos: fecha, usuario, criptomoneda, tipo: compra de cripto, monto de cripto y cotización.

➢ Vender determinada criptomoneda: dado un monto de cripto se vende por fiat,
    tenga en cuenta que al momento de realizar la operación se obtiene del sistema la cotización actual
    de la criptomoneda para acreditar la correspondiente proporción en el balance de fiat
    y desacreditar en el balance de la criptomoneda.
        Luego de ello se registra la transacción con los siguientes datos:
            fecha, usuario, criptomoneda, tipo: venta de cripto, monto de cripto y cotización.

➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain
    se le descuenta del balance de dicha cripto al usuario el monto,
    la blockchain devuelve un hash que representa una transacción en ella
    (esto hágalo retornando el nombre de la blockchain + un número random).
    Luego se genera una transacción con los siguientes datos:
        fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto, cotización.

➢ Recibir criptomoneda de blockchain: dado un monto de una cripto y una blockchain se le acredita
    al balancede dicha cripto al usuario el monto. Luego se genera una transacción con los siguientes datos:
        fecha, usuario, tipo: recepción cripto, blockchain, cripto, monto, cotización.

➢ Retirar fiat por determinado medio: dado un monto de fiat se le descuenta dicho monto del balance
    al usuario y se genera una transacción con la siguiente información:
        fecha, usuario, tipo: retiro fiat, monto y medio (puede ser MercadoPago o Transferencia Bancaria)
N
    ota:: Tanto para comprar. vender, retirar el usuario debe estar validado.
        Se debe validar siempre que haya balance suficiente para realizar la operación
        en los casos de compra, venta, retiro.


    Además la empresa desea saber lo siguiente en base a sus operaciones:

➢ Saber cual es la criptomoneda que más cantidad de ventas tiene
➢ Saber cual es la criptomoneda que más cantidad de compras tiene
➢ Saber cual es la criptomoneda que más volumen de ventas tiene
➢ Saber cual es la criptomoneda que más volumen de compras tiene

 */

//    De los usuarios se conoce:
//         nombre, apellido, email, dni, y si está validada su identidad o no.
//     Cada usuario tiene un balance de las criptomonedas que se ofrecen en la plataforma.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{AddAssign, SubAssign};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Balance(pub f64);

impl Balance {
    pub fn new(balance: f64) -> Self {
        Balance(balance)
    }
    pub fn add_assign_f64(&mut self, val: f64) {
        self.0+= val;
    }
    pub fn sub_assign_f64(&mut self, val: f64) {
        self.0-= val;
    }
    pub fn f64(self) -> f64 { self.0 }
}

impl Default for Balance {
    fn default() -> Self {
        Balance::from(0.0)
    }
}

impl Hash for Balance {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}
impl AddAssign for Balance {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl SubAssign for Balance {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl From<f64> for Balance {
    fn from(value: f64) -> Self {
        Balance(value)
    }
}

pub trait AsBalance {
    fn as_balance(&self) -> Balance;
}
impl AsBalance for f64 {
    fn as_balance(&self) -> Balance {
        Balance(self.clone())
    }
}

// user

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub id: u32, // primary key
    pub fiat_balance: Balance,
    pub crypto_balance: HashMap<String, Balance>
}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first_name.hash(state);
        self.last_name.hash(state);
        self.email.hash(state);
        self.id.hash(state);
        self.fiat_balance.hash(state);
    }
}