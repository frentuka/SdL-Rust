//     Cada usuario tiene un balance de las criptomonedas que se ofrecen en la plataforma.
//     De las criptomonedas se conoce:
//         nombre, prefijo y un listado de blockchains donde se pueden enviar o recibir.
//     De cada blockchain se conoce el nombre, prefijo.

use core::fmt;
use std::fmt::{Formatter};
use error_proc_macro::Error;
use serde::{Deserialize, Serialize};
use crate::structs::date::Date;

#[derive(Debug, PartialEq)]
pub struct BlockchainTransactionHash(String);
impl BlockchainTransactionHash {
    fn new(prefix: &str) -> BlockchainTransactionHash {
        BlockchainTransactionHash(format!("{}-{}", prefix, rand::random::<u32>()))
    }
}

#[derive(Debug, PartialEq)]
pub struct BlockchainTransaction {
    pub data: CommonTransactionData,
    pub blockchain: String,
    pub hash: BlockchainTransactionHash,
    pub crypto: String,
    pub quote: Quote
}

impl BlockchainTransaction {
    pub fn new(data: CommonTransactionData, blockchain: &str, hash: Option<BlockchainTransactionHash>, crypto: &str, quote: Quote) -> Result<Self, ErrorNewTransaction> {
        // invalid date
        if !data.date.is_date_valid() { return Err(ErrorNewTransaction::InvalidDate) }

        // invalid amount
        if data.amount < 0.0 { return Err(ErrorNewTransaction::InvalidInputAmount { amount: data.amount }) }

        // invalid transaction type
        if data.transaction_type != TransactionType::BlockchainWithdrawal
        && data.transaction_type != TransactionType::BlockchainDeposit
            { return Err(ErrorNewTransaction::InvalidTransactionType { transaction_type: data.transaction_type }) }

        // unwrap or create
        let hash = if let Some(val) = hash { val }
                                         else { BlockchainTransactionHash::new(&blockchain) };

        Ok(Self {
            data,
            blockchain: blockchain.to_string(),
            hash,
            crypto: crypto.to_string(),
            quote
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Blockchain {
    pub name: String,
    pub prefix: String,
    pub supported_cryptos: Vec<String>,
}

impl Blockchain {
    fn new(name: &str, prefix: &str, supported_cryptos: Vec<String>) -> Self {
        Blockchain { name: name.to_string(), prefix: prefix.to_string(), supported_cryptos }
    }

    // ➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain
    // se le descuenta del balance de dicha cripto al usuario el monto,
    // la blockchain devuelve un hash que representa una transacción en ella
    // (esto hágalo retornando el nombre de la blockchain + un número random).
    // Luego se genera una transacción con los siguientes datos:
    // fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto, cotización.

    fn withdraw(&self, data: CommonTransactionData, crypto: &str, quote: Quote) -> Result<BlockchainTransaction, ErrorNewTransaction> {
        if !self.supported_cryptos.contains(&crypto.to_string()) {
            return Err(ErrorNewTransaction::CryptoNotSupportedByBlockchain { crypto: crypto.to_string(), blockchain: self.name.clone() })
        }
        
        // all other checks are made by BlockchainTransaction::new()
        BlockchainTransaction::new(
            data,
            self.name.as_str(),
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

// Quote
// I could use a tuple instead of a whole struct,
// but I want to enforce compile-time names for values
// as it's not intuitive that .0 is the BUY value and .1 the SELL value
// quote must be copied,
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Quote {
    pub buy: f64,
    pub sell: f64
}

// CommonTransactionData
// It's only purpose is to prevent having too many arguments.
// It's only supposed to have data which all transactions need.
// TransctionType could also be set here, but that would imply that user must set the transaction type
// and I prefer transaction types to be hard-coded.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct CommonTransactionData {
    pub date: Date,
    pub user: u32,
    pub amount: f64,
    pub transaction_type: TransactionType,
}

// ➢ Ingresar dinero: se recibe un monto en fiat de un usuario
//     y se acredita al balance de fiat de dicho usuario. Además se crea una transacción del hecho
//      donde los datos que se guardan son:fecha, tipo(ingreso de dinero), monto, usuario.

#[derive(Error, PartialEq)]
pub enum ErrorNewTransaction {
    InvalidDate,
    InvalidInputAmount{ amount: f64 },
    InvalidTransactionType { transaction_type: TransactionType },
    BlockchainNotDeclared,
    CryptoNotSupportedByBlockchain { crypto: String, blockchain: String },
    FiatWithdrawalNeedsMean
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum WithdrawalMean {
    BankTansfer, MercadoPago
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Copy)]
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
            TransactionType::FiatWithdrawal { mean } => write!(f, "Fiat Withdrawal via {mean:?}"),
            TransactionType::BlockchainDeposit => write!(f, "Blockchain Deposit"),
            TransactionType::BlockchainWithdrawal => write!(f, "Blockchain Withdrawal"),
            TransactionType::CryptoBuy => write!(f, "Crypto Buy"),
            TransactionType::CryptoSell => write!(f, "Crypto Sell"),
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct FiatTransaction {
    pub data: CommonTransactionData,
}

// all FIAT transfers will be treated as Argentine Peso transfers

impl FiatTransaction {
    pub fn new(data: CommonTransactionData) -> Result<Self, ErrorNewTransaction> {
        match data.transaction_type {
            TransactionType::FiatDeposit | TransactionType::FiatWithdrawal { .. } => (),
            _ => return Err(ErrorNewTransaction::InvalidTransactionType { transaction_type: data.transaction_type })
        }
        
        if !data.date.is_date_valid() { return Err(ErrorNewTransaction::InvalidDate) }
        if data.amount <= 0.0 { return Err(ErrorNewTransaction::InvalidInputAmount{ amount: data.amount }) }

        // user verifications must be done service-side

        Ok(FiatTransaction {
            data
        })
    }
}

//
// Crypto Transaction
//

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CryptoTransaction {
    pub data: CommonTransactionData,
    pub currency: String,
}

// all FIAT transfers will all be treated as Argentine Peso transfers

impl CryptoTransaction {
    pub fn new(data: CommonTransactionData, currency: &str) -> Result<Self, ErrorNewTransaction> {
        if !data.date.is_date_valid() { return Err(ErrorNewTransaction::InvalidDate) }
        if data.amount < 0.0 { return Err(ErrorNewTransaction::InvalidInputAmount{ amount: data.amount }) }

        match data.transaction_type {
            TransactionType::CryptoBuy | TransactionType::CryptoSell => (),
            _ => { return Err(ErrorNewTransaction::InvalidTransactionType { transaction_type: data.transaction_type }) }
        }
        
        // blockchain, currency, user_from, user_to verifications must be done service-side

        Ok(CryptoTransaction {
            data, currency: currency.to_string()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_blockchain() {
        let blockchain = Blockchain::new("Ethereum", "ETH", vec!["ETH".to_string(), "USDT".to_string()]);

        assert_eq!(blockchain.name, "Ethereum");
        assert_eq!(blockchain.prefix, "ETH");
        assert_eq!(blockchain.supported_cryptos, vec!["ETH".to_string(), "USDT".to_string()]);
    }

    #[test]
    fn test_blockchain_withdraw() {
        let blockchain = Blockchain::new("Ethereum", "ETH", vec!["ETH".to_string(), "USDT".to_string()]);

        let data = CommonTransactionData {
            date: Date::new(2, 10, 1).unwrap(),
            user: 1,
            amount: 100.0,
            transaction_type: TransactionType::BlockchainWithdrawal
        };

        let quote = Quote { buy: 2000.0, sell: 1900.0 };

        let transaction = blockchain.withdraw(data, "ETH", quote.clone()).unwrap();

        assert_eq!(transaction.blockchain, "Ethereum");
        assert_eq!(transaction.crypto, "ETH");
        assert_eq!(transaction.quote, quote);
    }

    #[test]
    fn test_transactiontype_display_impl() {
        let withdrawal = TransactionType::FiatWithdrawal { mean: WithdrawalMean::BankTansfer };
        let withdrawal_str = format!("{}", withdrawal);
        assert_eq!(withdrawal_str, "Fiat Withdrawal via BankTansfer");

        let deposit = TransactionType::FiatDeposit;
        let deposit_str = format!("{}", deposit);
        assert_eq!(deposit_str, "Fiat Deposit");

        let blockchain_deposit = TransactionType::BlockchainDeposit;
        let blockchain_deposit_str = format!("{}", blockchain_deposit);
        assert_eq!(blockchain_deposit_str, "Blockchain Deposit");

        let blockchain_withdrawal = TransactionType::BlockchainWithdrawal;
        let blockchain_withdrawal_str = format!("{}", blockchain_withdrawal);
        assert_eq!(blockchain_withdrawal_str, "Blockchain Withdrawal");

        let crypto_buy = TransactionType::CryptoBuy;
        let crypto_buy_str = format!("{}", crypto_buy);
        assert_eq!(crypto_buy_str, "Crypto Buy");

        let crypto_sell = TransactionType::CryptoSell;
        let crypto_sell_str = format!("{}", crypto_sell);
        assert_eq!(crypto_sell_str, "Crypto Sell");
    }
}