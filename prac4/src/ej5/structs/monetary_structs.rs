//     Cada usuario tiene un balance de las criptomonedas que se ofrecen en la plataforma.
//     De las criptomonedas se conoce:
//         nombre, prefijo y un listado de blockchains donde se pueden enviar o recibir.
//     De cada blockchain se conoce el nombre, prefijo.

use core::fmt;
use std::fmt::{format, Formatter};
use ErrorProcMacro::Error;
use crate::structs::date::Date;
use crate::structs::monetary_structs::ErrorNewTransaction::NotAFiatOperation;

pub struct Blockchain<'a> {
    pub name: &'a str,
    pub prefix: &'a str
}

impl<'a> Blockchain<'a> {
    fn new(name: &'a str, prefix: &'a str) -> Self {
        Blockchain { name, prefix }
    }

    // ➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain
    // se le descuenta del balance de dicha cripto al usuario el monto,
    // la blockchain devuelve un hash que representa una transacción en ella
    // (esto hágalo retornando el nombre de la blockchain + un número random).
    // Luego se genera una transacción con los siguientes datos:
    // fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto, cotización.

    fn
}

//     Cada usuario tiene un balance de las criptomonedas que se ofrecen en la plataforma.
//     De las criptomonedas se conoce:
//         nombre, prefijo y un listado de blockchains donde se pueden enviar o recibir.
//     De cada blockchain se conoce el nombre, prefijo.

pub enum ErrorNewCryptocurrency {
    MustHaveABlockchain
}

pub struct Cryptocurrency<'a> {
    pub name: &'a str,
    pub prefix: &'a str,
    pub blockchains: Vec<&'a str> // blockchains prefix
}

impl<'a> Cryptocurrency<'a> {
    pub fn new(name: &'a str, prefix: &'a str, blockchains: Vec<&'a str>) -> Result<Self, ErrorNewCryptocurrency> {
        if blockchains.is_empty() { return Err(ErrorNewCryptocurrency::MustHaveABlockchain) }
        Ok(Cryptocurrency {
            name, prefix, blockchains
        })
    }
}

// Quote
// I could use a tuple instead of a whole struct,
// but I want to enforce compile-time names for values
// as it's not intuitive that .0 is BUY value and .1 is SELL value
pub struct Quote {
    pub buy: f64,
    pub sell: f64
}

// ➢ Ingresar dinero: se recibe un monto en fiat de un usuario
//     y se acredita al balance de fiat de dicho usuario. Además se crea una transacción del hecho
//      donde los datos que se guardan son:fecha, tipo(ingreso de dinero), monto, usuario.

#[derive(Error)]
pub enum ErrorNewTransaction {
    InvalidDate,
    InvalidInputAmount{ amount: f64 },
    NotAFiatOperation{ operation: TransactionType },
    BlockchainNotDeclared
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WithdrawalMean {
    BankTansfer, MercadoPago
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TransactionType {
    FiatDeposit,
    FiatWithdrawal { mean: WithdrawalMean },
    BlockchainDeposit,
    BlockchainWithdrawal,
    CryptoBuy,
    CryptoSell
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::FiatDeposit => write!(f, "Fiat Deposit"),
            TransactionType::FiatWithdrawal { mean } => write!(f, "Fiat Withdrawal via {:?}", mean),
            TransactionType::BlockchainDeposit => write!(f, "Blockchain Deposit"),
            TransactionType::BlockchainWithdrawal => write!(f, "Blockchain Withdrawal"),
            TransactionType::CryptoBuy => write!(f, "Crypto Buy"),
            TransactionType::CryptoSell => write!(f, "Crypto Sell"),
        }
    }
}

pub struct FiatTransaction {
    pub date: Date,
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub user: u32
}

// all FIAT transfers will all be treated as Argentine Peso transfers

impl FiatTransaction {
    pub fn new(date: Date, transaction_type: TransactionType, amount: f64, user: u32) -> Result<Self, ErrorNewTransaction> {
        match transaction_type {
            TransactionType::FiatDeposit => {},
            TransactionType::FiatWithdrawal { .. } => {},
            operation => return Err(NotAFiatOperation{ operation })
        }
        if !date.is_date_valid() { return Err(ErrorNewTransaction::InvalidDate) }
        if amount <= 0.0 { return Err(ErrorNewTransaction::InvalidInputAmount{ amount }) }

        // user_from, user_to verifications must be done service-side

        Ok(FiatTransaction {
            date, transaction_type, amount, user
        })
    }
}



pub struct CryptoTransaction<'a> {
    pub date: Date,
    pub transaction_type: TransactionType,
    pub currency: &'a str,
    pub amount: f64,
    pub user: u32
}

// all FIAT transfers will all be treated as Argentine Peso transfers

impl<'a> CryptoTransaction<'a> {
    pub fn new(date: Date, transaction_type: TransactionType, currency: &'a str, amount: f64, user: u32) -> Result<Self, ErrorNewTransaction> {
        if !date.is_date_valid() { return Err(ErrorNewTransaction::InvalidDate) }
        if amount < 0.0 { return Err(ErrorNewTransaction::InvalidInputAmount{ amount }) }

        // blockchain, currency, user_from, user_to verifications must be done service-side


        Ok(CryptoTransaction {
            date, transaction_type, currency, amount, user
        })
    }
}