//     Cada usuario tiene un balance de las criptomonedas que se ofrecen en la plataforma.
//     De las criptomonedas se conoce:
//         nombre, prefijo y un listado de blockchains donde se pueden enviar o recibir.
//     De cada blockchain se conoce el nombre, prefijo.

use core::fmt;
use std::fmt::{Formatter};
use error_proc_macro::Error;
use crate::structs::date::Date;

pub struct BlockchainTransactionHash(String);
impl BlockchainTransactionHash {
    fn new(prefix: &str) -> BlockchainTransactionHash {
        BlockchainTransactionHash(format!("{}-{}", prefix, rand::random::<u32>()))
    }
}

pub struct BlockchainTransaction {
    pub data: CommonTransactionData,
    pub blockchain: String,
    pub hash: BlockchainTransactionHash,
    pub crypto: String,
    pub quote: Quote
}

impl BlockchainTransaction {
    pub fn new(data: CommonTransactionData, transaction_type: TransactionType, blockchain: &str, hash: Option<BlockchainTransactionHash>, crypto: &str, quote: Quote) -> Result<Self, ErrorNewTransaction> {
        // invalid date
        if !data.date.is_date_valid() { return Err(ErrorNewTransaction::InvalidDate) }

        // invalid amount
        if data.amount < 0.0 { return Err(ErrorNewTransaction::InvalidInputAmount { amount: data.amount }) }

        // invalid transaction type
        if transaction_type != TransactionType::BlockchainWithdrawal
        && transaction_type != TransactionType::BlockchainDeposit
            { return Err(ErrorNewTransaction::InvalidTransactionType { transaction_type }) }

        // unwrap or create
        let hash = if let Some(val) = hash { val }
                                         else { BlockchainTransactionHash::new(blockchain) };

        Ok(Self {
            data,
            blockchain: blockchain.to_string(),
            hash,
            crypto: crypto.to_string(),
            quote
        })
    }
}

pub struct Blockchain {
    pub name: String,
    pub prefix: String,
    pub supported_cryptos: Vec<String>,
}

impl Blockchain {
    fn new(name: &str, prefix: &str, supported_cryptos: Vec<String>) -> Self {
        Blockchain {
            name: name.to_string(),
            prefix: prefix.to_string(),
            supported_cryptos
        }
    }

    // ➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain
    // se le descuenta del balance de dicha cripto al usuario el monto,
    // la blockchain devuelve un hash que representa una transacción en ella
    // (esto hágalo retornando el nombre de la blockchain + un número random).
    // Luego se genera una transacción con los siguientes datos:
    // fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto, cotización.

    fn withdraw(&self, data: CommonTransactionData, transaction_type: TransactionType, crypto: &str, quote: Quote) -> Result<BlockchainTransaction, ErrorNewTransaction> {
        if !self.supported_cryptos.contains(&crypto.to_string()) {
            return Err(ErrorNewTransaction::CryptoNotSupportedByBlockchain {
                crypto: crypto.to_string(),
                blockchain: self.name.to_string()
            }) }
        
        // all other checks are made by BlockchainTransaction::new()
        BlockchainTransaction::new(
            data,
            transaction_type,
            &self.name,
            None, // hash
            crypto,
            quote
        )
    }
}

//     Cada usuario tiene un balance de las criptomonedas que se ofrecen en la plataforma.
//     De las criptomonedas se conoce:
//         nombre, prefijo y un listado de blockchains donde se pueden enviar o recibir.
//     De cada blockchain se conoce el nombre, prefijo.

pub enum ErrorNewCryptocurrency {
    MustHaveABlockchain
}

pub struct Cryptocurrency {
    pub name: String,
    pub prefix: String,
    pub blockchains: Vec<String> // blockchains prefix
}

impl Cryptocurrency {
    pub fn new(name: &str, prefix: &str, blockchains: Vec<String>) -> Result<Self, ErrorNewCryptocurrency> {
        if blockchains.is_empty() { return Err(ErrorNewCryptocurrency::MustHaveABlockchain) }
        Ok(Cryptocurrency {
            name: name.to_string(),
            prefix: prefix.to_string(),
            blockchains
        })
    }
}

// Quote
// I could use a tuple instead of a whole struct,
// but I want to enforce compile-time names for values
// as it's not intuitive that .0 is the BUY value and .1 the SELL value
// quote must be copied,
#[derive(Clone)]
pub struct Quote {
    pub buy: f64,
    pub sell: f64
}

// CommonTransactionData
// It's only purpose is to prevent having too many arguments.
// It's only supposed to have data which all transactions need.
// TransctionType could also be set here, but that would imply that user must set the transaction type
// and I prefer transaction types to be hard-coded.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CommonTransactionData {
    pub date: Date,
    pub user: u32,
    pub amount: f64,
}

// ➢ Ingresar dinero: se recibe un monto en fiat de un usuario
//     y se acredita al balance de fiat de dicho usuario. Además se crea una transacción del hecho
//      donde los datos que se guardan son:fecha, tipo(ingreso de dinero), monto, usuario.

#[derive(Error)]
pub enum ErrorNewTransaction {
    InvalidDate,
    InvalidInputAmount{ amount: f64 },
    InvalidTransactionType { transaction_type: TransactionType },
    BlockchainNotDeclared,
    CryptoNotSupportedByBlockchain { crypto: String, blockchain: String },
    FiatWithdrawalNeedsMean
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum WithdrawalMean {
    BankTansfer, MercadoPago
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
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
    pub data: CommonTransactionData,
    pub transaction_type: TransactionType
}

// all FIAT transfers will be treated as Argentine Peso transfers

impl FiatTransaction {
    pub fn new(data: CommonTransactionData, transaction_type: TransactionType) -> Result<Self, ErrorNewTransaction> {
        match transaction_type {
            TransactionType::FiatDeposit => (),
            TransactionType::FiatWithdrawal { .. } => (),
            _ => return Err(ErrorNewTransaction::InvalidTransactionType { transaction_type })
        }
        
        if !data.date.is_date_valid() { return Err(ErrorNewTransaction::InvalidDate) }
        if data.amount <= 0.0 { return Err(ErrorNewTransaction::InvalidInputAmount{ amount: data.amount }) }

        // user verifications must be done service-side

        Ok(FiatTransaction {
            data, transaction_type
        })
    }
}

//
// Crypto Transaction
//

pub struct CryptoTransaction {
    pub data: CommonTransactionData,
    pub currency: String,
}

// all FIAT transfers will all be treated as Argentine Peso transfers

impl CryptoTransaction {
    pub fn new(data: CommonTransactionData, transaction_type: TransactionType, currency: &str) -> Result<Self, ErrorNewTransaction> {
        if !data.date.is_date_valid() { return Err(ErrorNewTransaction::InvalidDate) }
        if data.amount < 0.0 { return Err(ErrorNewTransaction::InvalidInputAmount{ amount: data.amount }) }

        match transaction_type {
            TransactionType::CryptoBuy => (),
            TransactionType::CryptoSell => (),
            _ => { return Err(ErrorNewTransaction::InvalidTransactionType { transaction_type }) }
        }
        
        // blockchain, currency, user_from, user_to verifications must be done service-side


        Ok(CryptoTransaction {
            data,
            currency: currency.to_string()
        })
    }
}