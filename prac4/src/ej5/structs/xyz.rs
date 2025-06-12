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

Nota:: Tanto para comprar. vender, retirar el usuario debe estar validado.
Se debe validar siempre que haya balance suficiente para realizar la operación
en los casos de compra, venta, retiro.


Además la empresa desea saber lo siguiente en base a sus operaciones:

➢ Saber cual es la criptomoneda que más cantidad de ventas tiene
➢ Saber cual es la criptomoneda que más cantidad de compras tiene
➢ Saber cual es la criptomoneda que más volumen de ventas tiene
➢ Saber cual es la criptomoneda que más volumen de compras tiene

*/
use std::cmp::{Ordering, PartialOrd};
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use ErrorProcMacro::Error;
use crate::structs::date::Date;
use crate::structs::monetary_structs::{Blockchain, CryptoTransaction, ErrorNewTransaction, FiatTransaction, Quote, TransactionType};
use crate::structs::user::{FiatBalance, User};

pub struct XYZ<'a> {
    pub users: BTreeMap<u32, User<'a>>,
    pub blockchains: BTreeMap<&'a str, Blockchain<'a>>,
    pub quotes: HashMap<&'a str, Quote> // (buy, sell) prices
}

//
// errors
//

#[derive(Error)]
pub enum ErrorDeposit {
    FiatTransactionError(ErrorNewTransaction),
    UserNotFound{ user_dni: u32 }
}

#[derive(Error)]
pub enum ErrorBuySell<'a> {
    CryptoTransactionError(ErrorNewTransaction),
    CryptocurrencyNotQuoted { crypto_prefix: &'a str },
    UserNotFound { user_dni: u32 },
    NotEnoughBalance { balance: f64, balance_needed: f64 },
    NegativeAmount,
    Unknown(String)
}

impl<'a> XYZ<'a> {
    fn new(users: BTreeMap<u32, User<'a>>, blockchains: BTreeMap<&'a str, Blockchain<'a>>, quotes: HashMap<&'a str, Quote>) -> Self {
        XYZ { users, blockchains, quotes }
    }

    // ➢ Ingresar dinero: se recibe un monto en fiat de un usuario
    //  y se acredita al balance de fiat de dicho usuario. Además se crea una transacción del hecho.
    fn deposit(&mut self, date: Date, user_dni: u32, fiat_amount: f64) -> Result<FiatTransaction, ErrorDeposit> {
        // date errors are handled by FiatTransaction::new()
        match FiatTransaction::new(
            date,
            TransactionType::FiatDeposit,
            fiat_amount,
            user_dni
        ) {
            Ok(transaction) => {
                // deposit
                if let Some(user) = self.users.get_mut(&user_dni) {
                    user.fiat_balance += FiatBalance::from(fiat_amount);
                } else {
                    return Err(ErrorDeposit::UserNotFound{user_dni});
                }

                Ok(transaction)
            },
            Err(transaction_error) => Err(ErrorDeposit::FiatTransactionError(transaction_error))
        }
    }

    // ➢ Comprar determinada criptomoneda: dado un monto de fiat se compra una cantidad de determinada criptomoneda,
    //  tenga en cuenta que al momento de realizar la operación se obtiene del sistema
    //  la cotización actual de la criptomoneda para acreditar la correspondiente proporción en el balance
    //  de la cripto y desacreditar en el balance de fiat.
    // Luego de ello se registra la transacción con los siguientes datos:
    //      fecha, usuario, criptomoneda, tipo: compra de cripto, monto de cripto y cotización.
    fn buy_crypto(&mut self, date: Date, user_dni: u32, crypto_prefix: &'a str, fiat_amount: f64)
                  -> Result<CryptoTransaction<'a>, ErrorBuySell> {
        // date errors are handled by CryptoTransaction::new()

        // check 1: invalid fiat amount
        if fiat_amount < 0.0 {
            return Err(ErrorBuySell::NegativeAmount)
        }

        match CryptoTransaction::new(
            date,
            TransactionType::CryptoBuy,
            crypto_prefix,
            fiat_amount,
            user_dni
        ) {
            Ok(transaction) => {
                // process buy

                // check 2: currency must be quoted AND quoted higher than 0.0
                let currency_unitary_value = if let Some(quoting) = self.quotes.get(crypto_prefix) {
                    if quoting.buy <= 0.0 { return Err(ErrorBuySell::Unknown(format!("${crypto_prefix} is valued at {} FIAT (which is <= 0)", quoting.buy))) }
                    quoting.buy
                } else {
                    return Err(ErrorBuySell::CryptocurrencyNotQuoted{ crypto_prefix });
                };

                let transaction_crypto_amount = fiat_amount / currency_unitary_value; // .0 -> buy, .1 -> sell

                // check 3: user must exist
                if let Some(user) = self.users.get_mut(&user_dni) {
                    // check 4: user must have enough fiat balance
                    if user.fiat_balance < FiatBalance::from(fiat_amount) {
                        return Err(ErrorBuySell::NotEnoughBalance{ balance: user.fiat_balance.into(), balance_needed: fiat_amount })
                    }

                    // no error. execute operation
                    user.fiat_balance-= FiatBalance::from(fiat_amount);
                    *user.crypto_balance.0.entry(crypto_prefix).or_insert(0.0)+= transaction_crypto_amount;
                } else {
                    return Err(ErrorBuySell::UserNotFound{ user_dni });
                };

                Ok(transaction)
            },
            Err(error) => Err(ErrorBuySell::CryptoTransactionError(error))
        }
    }

    // ➢ Vender determinada criptomoneda: dado un monto de cripto se vende por fiat,
    //  tenga en cuenta que al momento de realizar la operación se obtiene del sistema la cotización actual
    //  de la criptomoneda para acreditar la correspondiente proporción en el balance de fiat
    //  y desacreditar en el balance de la criptomoneda.
    //  Luego de ello se registra la transacción con los siguientes datos:
    //  fecha, usuario, criptomoneda, tipo: venta de cripto, monto de cripto y cotización.
    fn sell_crypto(&mut self, date: Date, user_dni: u32, crypto_prefix: &'a str, crypto_amount: f64) ->
        Result<CryptoTransaction, ErrorBuySell> {
        // date errors are handled by CryptoTransaction::new()

        // check 1: amounts should be higher than 0
        if crypto_amount <= 0.0 {
            return Err(ErrorBuySell::NegativeAmount)
        }

        match CryptoTransaction::new(
            date,
            TransactionType::CryptoSell,
            crypto_prefix,
            crypto_amount,
            user_dni
        ) {
            Ok(transaction) => {
                // process sell

                // check 2: currency must be quoted AND quoted higher than 0.0
                let currency_value = if let Some(quoting) = self.quotes.get(crypto_prefix) {
                    if quoting.sell <= 0.0 { return Err(ErrorBuySell::Unknown(format!("${crypto_prefix} is valued at {} FIAT (which is <= 0)", quoting.sell))) }
                    quoting
                } else {
                    return Err(ErrorBuySell::CryptocurrencyNotQuoted{ crypto_prefix });
                };

                let transaction_fiat_value = currency_value.sell * crypto_amount;

                // check 3: user must exist
                if let Some(user) = self.users.get_mut(&user_dni) {
                    // check 4: user must have enough $crypto_prefix balance
                    if let Some(user_crypto_balance) = user.crypto_balance.0.get_mut(crypto_prefix) {
                        if *user_crypto_balance < crypto_amount {
                            return Err(ErrorBuySell::NotEnoughBalance { balance: *user_crypto_balance, balance_needed: crypto_amount })
                        }
                        
                        // no error. execute operation
                        *user_crypto_balance-= crypto_amount;
                        user.fiat_balance+= FiatBalance::from(transaction_fiat_value);
                    } else {
                        return Err(ErrorBuySell::NotEnoughBalance{ balance: 0.0, balance_needed: crypto_amount })
                    }
                } else {
                    return Err(ErrorBuySell::UserNotFound{ user_dni });
                };

                Ok(transaction)
            },
            Err(error) => Err(ErrorBuySell::CryptoTransactionError(error))
        }
    }

    // ➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain
    // se le descuenta del balance de dicha cripto al usuario el monto,
    // la blockchain devuelve un hash que representa una transacción en ella
    // (esto hágalo retornando el nombre de la blockchain + un número random).
    // Luego se genera una transacción con los siguientes datos:
    // fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto, cotización.

    // ➢ Recibir criptomoneda de blockchain: dado un monto de una cripto y una blockchain se le acredita
    // al balancede dicha cripto al usuario el monto. Luego se genera una transacción con los siguientes datos:
    // fecha, usuario, tipo: recepción cripto, blockchain, cripto, monto, cotización.

    // ➢ Retirar fiat por determinado medio: dado un monto de fiat se le descuenta dicho monto del balance
    // al usuario y se genera una transacción con la siguiente información:
    // fecha, usuario, tipo: retiro fiat, monto y medio (puede ser MercadoPago o Transferencia Bancaria)

    // Nota:: Tanto para comprar. vender, retirar el usuario debe estar validado.
    // Se debe validar siempre que haya balance suficiente para realizar la operación
    // en los casos de compra, venta, retiro.
}