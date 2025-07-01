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
use error_proc_macro::Error;
use std::collections::{BTreeMap, HashMap};
use crate::structs::user::{Balance, User};
use crate::structs::monetary_structs::{Blockchain, BlockchainTransaction, CommonTransactionData, CryptoTransaction, ErrorNewTransaction, FiatTransaction, Quote, TransactionType, WithdrawalMean};

pub struct XYZ {
    pub users: BTreeMap<u32, User>,
    pub blockchains: BTreeMap<String, Blockchain>,
    pub quotes: HashMap<String, Quote> // (buy, sell) prices
}

//
// errors
//

#[derive(Error)]
pub enum ErrorFiatDeposit {
    FiatTransactionError(ErrorNewTransaction),
    UserNotFound{ user_dni: u32 }
}

#[derive(Error)]
pub enum ErrorFiatWithdraw {
    FiatTransactionError(ErrorNewTransaction),
    UserNotFound{ user_dni: u32 },
    NotEnoughBalance{ balance: f64, balance_needed: f64 },
}

#[derive(Error)]
pub enum ErrorBlockchainDeposit {
    BlockchainTransactionError(ErrorNewTransaction),
    BlockchainNotFound{ blockchain: String },
    CryptoNotQuoted{ crypto: String },
    UserNotFound{ user_dni: u32 },
}

#[derive(Error)]
pub enum ErrorBlockchainWithdraw {
    BlockchainTransactionError(ErrorNewTransaction),
    BlockchainNotFound{ blockchain: String },
    CryptoNotQuoted{ crypto: String },
    UserNotFound{ user_dni: u32 },
    NotEnoughBalance{ balance: f64, balance_needed: f64 }
}

#[derive(Error)]
pub enum ErrorBuySell {
    CryptoTransactionError(ErrorNewTransaction),
    CryptocurrencyNotQuoted { crypto_prefix: String },
    UserNotFound { user_dni: u32 },
    NotEnoughBalance { balance: f64, balance_needed: f64 },
    NegativeAmount,
    Unknown(String)
}

impl XYZ {
    fn new(users: BTreeMap<u32, User>, blockchains: BTreeMap<String, Blockchain>, quotes: HashMap<String, Quote>) -> Self {
        Self { users, blockchains, quotes }
    }

    // ➢ Ingresar dinero: se recibe un monto en fiat de un usuario
    //  y se acredita al balance de fiat de dicho usuario. Además se crea una transacción del hecho.
    fn fiat_deposit(&mut self, data: CommonTransactionData) -> Result<FiatTransaction, ErrorFiatDeposit> {
        // date errors are handled by FiatTransaction::new()
        match FiatTransaction::new(
            data,
            TransactionType::FiatDeposit
        ) {
            Ok(transaction) => {
                // deposit
                if let Some(user) = self.users.get_mut(&data.user) {
                    user.fiat_balance += Balance::from(data.amount);
                } else {
                    return Err(ErrorFiatDeposit::UserNotFound{ user_dni: data.user });
                }

                Ok(transaction)
            },
            Err(transaction_error) => Err(ErrorFiatDeposit::FiatTransactionError(transaction_error))
        }
    }

    // ➢ Retirar fiat por determinado medio: dado un monto de fiat se le descuenta dicho monto del balance
    // al usuario y se genera una transacción con la siguiente información:
    // fecha, usuario, tipo: retiro fiat, monto y medio (puede ser MercadoPago o Transferencia Bancaria)
    fn fiat_withdraw(&mut self, data: CommonTransactionData, mean: WithdrawalMean) -> Result<FiatTransaction, ErrorFiatWithdraw> {

        match FiatTransaction::new(
            data,
            TransactionType::FiatWithdrawal{ mean }
        ) {
            Ok(transaction) => {
                // does user exist/have enough balance?
                if let Some(user) = self.users.get_mut(&data.user) {
                    // not enough! abort.
                    if user.fiat_balance < Balance(data.amount) { return Err( ErrorFiatWithdraw::NotEnoughBalance {
                        balance_needed: data.amount, balance: user.fiat_balance.f64()
                    }) }

                    // enough! substract balance
                    user.fiat_balance-= Balance(data.amount);
                } else {
                    return Err(ErrorFiatWithdraw::UserNotFound { user_dni: data.user })
                };

                Ok(transaction)
            }
            Err(error) => { Err(ErrorFiatWithdraw::FiatTransactionError( error )) }
        }
    }

    // ➢ Comprar determinada criptomoneda: dado un monto de fiat se compra una cantidad de determinada criptomoneda,
    //  tenga en cuenta que al momento de realizar la operación se obtiene del sistema
    //  la cotización actual de la criptomoneda para acreditar la correspondiente proporción en el balance
    //  de la cripto y desacreditar en el balance de fiat.
    // Luego de ello se registra la transacción con los siguientes datos:
    //      fecha, usuario, criptomoneda, tipo: compra de cripto, monto de cripto y cotización.
    fn buy_crypto(&mut self, data: CommonTransactionData, crypto_prefix: &str)
                  -> Result<CryptoTransaction, ErrorBuySell> {
        // date errors are handled by CryptoTransaction::new()

        // check 1: invalid fiat amount
        if data.amount < 0.0 {
            return Err(ErrorBuySell::NegativeAmount)
        }

        match CryptoTransaction::new(
            data,
            TransactionType::CryptoBuy,
            &crypto_prefix
        ) {

            Ok(transaction) => {
                // process buy

                // check 2: currency must be quoted AND quoted higher than 0.0
                let currency_unitary_value = if let Some(quoting) = self.quotes.get(crypto_prefix) {
                    if quoting.buy <= 0.0 { return Err(ErrorBuySell::Unknown(format!("${crypto_prefix} is valued at {} FIAT (which is <= 0)", quoting.buy))) }
                    quoting.buy
                } else {
                    return Err(ErrorBuySell::CryptocurrencyNotQuoted{ crypto_prefix: crypto_prefix.to_string() });
                };

                let transaction_crypto_amount = data.amount / currency_unitary_value; // .0 -> buy, .1 -> sell

                // check 3: user must exist
                if let Some(user) = self.users.get_mut(&data.user) {
                    // check 4: user must have enough fiat balance
                    if user.fiat_balance < Balance::from(data.amount) {
                        return Err(ErrorBuySell::NotEnoughBalance{ balance: user.fiat_balance.f64(), balance_needed: data.amount })
                    }

                    // no error. execute operation
                    user.fiat_balance-= Balance::from(data.amount);
                    *user.crypto_balance.entry(crypto_prefix.to_string()).or_insert(Balance::from(0.0))+= Balance::from(transaction_crypto_amount);
                } else {
                    return Err(ErrorBuySell::UserNotFound{ user_dni: data.user });
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
    fn sell_crypto(&mut self, data: CommonTransactionData, crypto_prefix: &str) ->
        Result<CryptoTransaction, ErrorBuySell> {
        // date errors are handled by CryptoTransaction::new()

        // check 1: amounts should be higher than 0
        if data.amount <= 0.0 {
            return Err(ErrorBuySell::NegativeAmount)
        }

        match CryptoTransaction::new(
            data,
            TransactionType::CryptoSell,
            crypto_prefix,
        ) {
            Ok(transaction) => {
                // process sell

                // check 2: currency must be quoted AND quoted higher than 0.0
                let currency_value = if let Some(quoting) = self.quotes.get(crypto_prefix) {
                    if quoting.sell <= 0.0 { return Err(ErrorBuySell::Unknown(format!("${crypto_prefix} is valued at {} FIAT (which is <= 0)", quoting.sell))) }
                    quoting
                } else {
                    return Err(ErrorBuySell::CryptocurrencyNotQuoted{ crypto_prefix: crypto_prefix.to_string() });
                };

                let transaction_fiat_value = currency_value.sell * data.amount;

                // check 3: user must exist
                if let Some(user) = self.users.get_mut(&data.user) {
                    // check 4: user must have enough $crypto_prefix balance
                    if let Some(user_crypto_balance) = user.crypto_balance.get_mut(crypto_prefix) {
                        if *user_crypto_balance < Balance::from(data.amount) {
                            return Err(ErrorBuySell::NotEnoughBalance { balance: user_crypto_balance.f64(), balance_needed: data.amount })
                        }
                        
                        // no error. execute operation
                        *user_crypto_balance-= Balance::from(data.amount);
                        user.fiat_balance+= Balance::from(transaction_fiat_value);
                    } else {
                        return Err(ErrorBuySell::NotEnoughBalance{ balance: 0.0, balance_needed: data.amount })
                    }
                } else {
                    return Err(ErrorBuySell::UserNotFound{ user_dni: data.user });
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
    fn withdraw_to_blockchain(&mut self, data: CommonTransactionData, blockchain: &str, crypto: &str) -> Result<BlockchainTransaction, ErrorBlockchainWithdraw> {
        // does blockchain exist?
        if !self.blockchains.contains_key(blockchain) {
            return Err(ErrorBlockchainWithdraw::BlockchainNotFound { blockchain: blockchain.to_string() })
        };

        // does crypto have a quote?
        let quote = if let Some(quote) = self.quotes.get(crypto) {
            quote
        } else {
            return Err(ErrorBlockchainWithdraw::CryptoNotQuoted { crypto: crypto.to_string() })
        };

        match BlockchainTransaction::new(
            data,
            TransactionType::BlockchainWithdrawal,
            blockchain,
            None,
            crypto,
            quote.clone() // quote should be cloned, as it changes over time. can't be copied due to containing f64
        ) {
            Ok(transaction) => {
                // remove balance
                // does user exist/have enough balance?
                if let Some(user) = self.users.get_mut(&data.user) {
                    if let Some(balance) = user.crypto_balance.get_mut(crypto) {
                        *balance-= Balance::from(data.amount);
                    } else {
                        return Err(ErrorBlockchainWithdraw::NotEnoughBalance { balance: 0.0, balance_needed: data.amount } )
                    };
                } else {
                    return Err(ErrorBlockchainWithdraw::UserNotFound { user_dni: data.user })
                };
                Ok(transaction)
            }
            Err(error) => { Err(ErrorBlockchainWithdraw::BlockchainTransactionError( error )) }
        }
    }

    // ➢ Recibir criptomoneda de blockchain: dado un monto de una cripto y una blockchain se le acredita
    // al balancede dicha cripto al usuario el monto. Luego se genera una transacción con los siguientes datos:
    // fecha, usuario, tipo: recepción cripto, blockchain, cripto, monto, cotización.
    fn deposit_from_blockchain(&mut self, data: CommonTransactionData, blockchain: &str, crypto: &str) -> Result<BlockchainTransaction, ErrorBlockchainDeposit> {
        // does blockchain exist?
        if !self.blockchains.contains_key(blockchain) {
            return Err(ErrorBlockchainDeposit::BlockchainNotFound { blockchain: blockchain.to_string() })
        };

        // does crypto have a quote?
        let quote = if let Some(q) = self.quotes.get(crypto) {
            q
        } else {
            return Err(ErrorBlockchainDeposit::CryptoNotQuoted { crypto: crypto.to_string() })
        };

        match BlockchainTransaction::new(
            data,
            TransactionType::BlockchainDeposit,
            blockchain,
            None,
            crypto,
            quote.clone() // quote should be cloned, as it changes over time. can't be copied due to containing f64
        ) {
            Ok(transaction) => {
                // add to balance
                // does user exist/have enough balance?
                if let Some(user) = self.users.get_mut(&data.user) {
                    // enough! ready to withdraw
                    user.fiat_balance+= Balance::from(data.amount);
                } else {
                    return Err(ErrorBlockchainDeposit::UserNotFound { user_dni: data.user })
                };

                Ok(transaction)
            }
            Err(error) => { Err(ErrorBlockchainDeposit::BlockchainTransactionError( error )) }
        }
    }

    // Nota:: Tanto para comprar. vender, retirar el usuario debe estar validado.
    // Se debe validar siempre que haya balance suficiente para realizar la operación
    // en los casos de compra, venta, retiro.
}