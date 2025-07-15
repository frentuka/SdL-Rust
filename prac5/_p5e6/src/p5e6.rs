


//
// date.rs
//

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{BTreeMap, HashMap};
use std::{fmt, fs};
use std::fmt::Formatter;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::ops::{AddAssign, SubAssign};
use serde::{Deserialize, Serialize};

const NOMBRE_MESES: [&str; 12] = ["Enero", "Febrero", "Marzo", "Abril",
    "Mayo", "Junio", "Julio", "Agosto",
    "Septiembre", "Octubre", "Noviembre", "Diciembre"];
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Copy, Hash)]
pub struct Date {
    pub(crate) day: u8,
    pub(crate) month: u8,
    pub(crate) year: i64
}

impl Default for Date {
    fn default() -> Self {
        Date { day: 1, month: 1, year: 0 }
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.day == other.day
            && self.month == other.month
            && self.year == other.year
        { return Some(Equal) }

        if self.year > other.year { return Some(Greater) }
        if self.month > other.month { return Some(Greater) }
        if self.day > other.day { return Some(Greater) }

        Some(Less)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_date_valid() {
            write!(f, "{} de {} del {}", self.day, NOMBRE_MESES[self.month as usize - 1], self.year)
        } else {
            write!(f, "{}/{}/{}", self.day, self.month, self.year)
        }
    }
}

impl Date {
    // El año podría ser negativo, indicando época antes de Cristo.
    pub fn new(dia: u8, mes: u8, ano: i64) -> Option<Date> {
        let fecha = Date { day: dia, month: mes, year: ano };
        if fecha.is_date_valid() {
            return Some(fecha);
        }
        None
    }

    pub fn is_date_valid(&self) -> bool {
        // check que el mes sea válido
        if !(1..=12).contains(&self.month) { return false }

        // check días del mes
        if self.day == 0
            || self.day > self.current_month_days()
        { return false }

        // el año no puede ser incorrecto...
        // a no ser que se contabilice la edad del universo
        // que dudo mucho que pueda importar para este caso
        true
    }

    pub fn is_leap_year(&self) -> bool {
        self.year % 4 == 0
    }

    pub fn add_days(&mut self, dias: u32) {
        let mut dias_restantes = dias;

        while dias_restantes > 0 {
            let dias_mes_actual = self.current_month_days();
            let dias_para_proximo_mes = u32::from(dias_mes_actual - self.day + 1);

            if dias_restantes >= dias_para_proximo_mes {
                // ir al siguiente mes

                dias_restantes-= dias_para_proximo_mes;
                self.day = 1;
                self.month += 1;

                if self.month > 12 {
                    self.month = 1;
                    self.year += 1;
                }
            } else {
                self.day += u8::try_from(dias_restantes).unwrap();
                dias_restantes = 0;
            }
        }
    }

    pub fn subtract_days(&mut self, dias: u32) {
        let mut dias_restantes = dias;

        while dias_restantes > 0 {
            if dias_restantes >= u32::from(self.day) {
                // ir al anterior mes
                dias_restantes-= u32::from(self.day);
                self.month -= 1;

                if self.month < 1 {
                    self.month = 12;
                    self.year -= 1;
                }

                // corregir self.dia == 0
                self.day = self.current_month_days();
            } else {
                self.day -= u8::try_from(dias_restantes).unwrap();
                dias_restantes = 0;
            }
        }
    }

    pub fn current_month_days(&self) -> u8 {
        match self.month {
            4 | 6 | 9 | 11 => 30,
            2 => if self.is_leap_year() { 29 } else { 28 },
            _ => 31,
        }
    }
}

#[cfg(test)]
mod test_date {
    use super::*;

    #[test]
    fn test_default() {
        // default: 01/01/0000
        let default_fecha = Date::default();
        assert_eq!(default_fecha.day, 1);
        assert_eq!(default_fecha.month, 1);
        assert_eq!(default_fecha.year, 0);
    }

    #[test]
    fn test_display() {
        let valid_fecha = Date::default();
        let invalid_fecha = Date { day: 0, month: 1, year: 0 };

        // write!(f, "{} de {} del {}", self.dia, NOMBRE_MESES[self.mes as usize - 1], self.ano)

        assert_ne!(format!("{valid_fecha}"), format!("{}", invalid_fecha));
        assert_eq!(format!("{valid_fecha}"), format!("{} de {} del {}", valid_fecha.day, NOMBRE_MESES[valid_fecha.month as usize - 1], valid_fecha.year));
        assert_eq!(format!("{invalid_fecha}"), format!("{}/{}/{}", invalid_fecha.day, invalid_fecha.month, invalid_fecha.year));
    }

    #[test]
    fn test_new() {
        // invalida
        let fecha = Date::new(0, 0, 0);
        assert!(fecha.is_none());

        // valida
        let fecha = Date::new(22, 08, 2002);
        assert!(fecha.is_some());
    }

    #[test]
    fn test_bisiesto() {
        let Some(fecha) = Date::new(1, 1, 0) else { panic!() };
        assert!(fecha.is_leap_year());

        let Some(fecha) = Date::new(1, 1, 2000) else { panic!() };
        assert!(fecha.is_leap_year());

        let Some(fecha) = Date::new(1, 1, -4) else { panic!() };
        assert!(fecha.is_leap_year());

        let Some(fecha) = Date::new(1, 1, 1) else { panic!() };
        assert!(!fecha.is_leap_year());
    }

    #[test]
    fn test_restar_dias() {
        let Some(mut fecha) = Date::new(30, 04, 2016) else { panic!() };

        fecha.subtract_days(5000);

        assert_eq!(fecha.day, 22);
        assert_eq!(fecha.month, 08);
        assert_eq!(fecha.year, 2002);
    }

    #[test]
    fn test_sumar_dias() {
        let Some(mut fecha) = Date::new(22, 08, 2002) else { panic!() };

        fecha.add_days(5000);

        assert_eq!(fecha.day, 30);
        assert_eq!(fecha.month, 04);
        assert_eq!(fecha.year, 2016);
    }

    #[test]
    fn test_current_month_days() {
        let Some(fecha) = Date::new(22, 01, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 31);
        let Some(fecha) = Date::new(22, 02, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 28);
        let Some(fecha) = Date::new(22, 02, 2004) else { panic!() };
        assert_eq!(fecha.current_month_days(), 29);
        let Some(fecha) = Date::new(22, 03, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 31);
        let Some(fecha) = Date::new(22, 04, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 30);
        let Some(fecha) = Date::new(22, 05, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 31);
        let Some(fecha) = Date::new(22, 06, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 30);
        let Some(fecha) = Date::new(22, 07, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 31);
        let Some(fecha) = Date::new(22, 08, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 31);
        let Some(fecha) = Date::new(22, 09, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 30);
        let Some(fecha) = Date::new(22, 10, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 31);
        let Some(fecha) = Date::new(22, 11, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 30);
        let Some(fecha) = Date::new(22, 12, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 31);
    }

    #[test]
    fn test_cmp() {
        let fecha1 = Date { day: 1, month: 1, year: 1};
        let fecha2 = Date { day: 3, month: 1, year: 1};
        let fecha3 = Date { day: 3, month: 1, year: 1};

        assert!(fecha1 < fecha2, "Fecha 1 es anterior, por ende, es menor");
        assert_eq!(fecha3, fecha2, "Fecha 3 es igual a fecha 2");
        assert!(fecha3 > fecha1, "Fecha 3 es posterior a fecha1, por ende, es mayor");
    }
}

//
// monetary_structs.rs
//

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

#[derive(Debug, Clone, PartialEq)]
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
mod test_monetary_structs {
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

//
// user.rs
//

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

//
// xyz.rs
//

type Users = BTreeMap<u32, User>;
type Blockchains = BTreeMap<String, Blockchain>;
type Quotes = HashMap<String, Quote>;
type CryptoTransactionHistory = BTreeMap<String, Vec<CryptoTransaction>>;

const BASE_FOLDER: &str = "";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct XYZ {
    file_name: String,
    pub users: Users,
    pub blockchains: Blockchains,
    pub quotes: Quotes, // (buy, sell) prices
    pub crypto_transactions: CryptoTransactionHistory, // <Prefix, Transactions>
}

//
// errors
//

#[derive(Debug, Clone, PartialEq)]
pub enum FiatDepositError {
    FiatTransactionError(ErrorNewTransaction),
    UserNotFound{ user_id: u32 },
    File(FileError)
}

#[derive(Debug, Clone, PartialEq)]
pub enum FiatWithdrawalError {
    FiatTransactionError(ErrorNewTransaction),
    UserNotFound{ user_id: u32 },
    NotEnoughBalance{ balance: f64, balance_needed: f64 },
    File(FileError)
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockchainDepositError {
    BlockchainTransactionError(ErrorNewTransaction),
    BlockchainNotFound{ blockchain: String },
    CryptoNotQuoted{ crypto: String },
    UserNotFound{ user_id: u32 },
    File(FileError)
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockchainWithdrawalError {
    BlockchainTransactionError(ErrorNewTransaction),
    BlockchainNotFound{ blockchain: String },
    CryptoNotQuoted{ crypto: String },
    UserNotFound{ user_id: u32 },
    NotEnoughBalance{ balance: f64, balance_needed: f64 },
    File(FileError)
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuySellError {
    TransactionError(ErrorNewTransaction),
    CryptocurrencyNotQuoted { crypto_prefix: String },
    UserNotFound { user_id: u32 },
    NotEnoughBalance { balance: f64, balance_needed: f64 },
    File(FileError),
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileError {
    Serialization,
    Deserialization,
    IO,
}

impl XYZ {
    fn new(file_name: Option<&str>) -> Self {
        let mut xyz = Self {
            file_name: file_name.unwrap_or("xyz").to_string(),
            users: Users::default(),
            blockchains: Blockchains::default(),
            quotes: Quotes::default(),
            crypto_transactions: CryptoTransactionHistory::default()
        };
        let _ = xyz.actualizar_datos_xyz();
        xyz
    }

    fn sobreescribir_archivo_xyz(&self) -> Result<(), FileError> {
        let parse_to_json: String = match serde_json::to_string_pretty(self) {
            Ok(json) => { json }
            Err(_) => { return Err(FileError::Serialization) }
        };

        match fs::write(format!("{BASE_FOLDER}{}.json", self.file_name), parse_to_json) {
            Ok(()) => { Ok(()) }
            Err(_) => { Err(FileError::IO) }
        }
    }

    fn leer_archivo_xyz(&self) -> Result<XYZ, FileError> {
        let Ok(mut file) = File::open(format!("{BASE_FOLDER}{}", self.file_name))
        else { return Err(FileError::IO) };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {},
            Err(_) => return Err(FileError::IO)
        }

        match serde_json::from_str(&contents) {
            Err(_) => Err(FileError::Deserialization),
            Ok(value) => Ok(value),
        }
    }

    fn actualizar_datos_xyz(&mut self) -> Result<(), FileError> {
        let data = self.leer_archivo_xyz()?;

        self.users = data.users;
        self.blockchains = data.blockchains;
        self.quotes = data.quotes;
        self.crypto_transactions = data.crypto_transactions;

        Ok(())
    }

    // ➢ Ingresar dinero: se recibe un monto en fiat de un usuario
    //  y se acredita al balance de fiat de dicho usuario. Además se crea una transacción del hecho.
    fn fiat_deposit(&mut self, today_date: Date, user_id: u32, amount: f64) -> Result<FiatTransaction, FiatDepositError> {
        let data = CommonTransactionData {
            date: today_date,
            user: user_id,
            amount,
            transaction_type: TransactionType::FiatDeposit,
        };

        // date errors are handled by FiatTransaction::new()
        match FiatTransaction::new(
            data,
        ) {
            Ok(transaction) => {
                // deposit
                if let Some(user) = self.users.get_mut(&data.user) {
                    user.fiat_balance += Balance::from(data.amount);
                } else {
                    return Err(FiatDepositError::UserNotFound{ user_id: data.user });
                }

                // guardar en archivo
                match self.sobreescribir_archivo_xyz() {
                    Ok(()) => {}
                    Err(err) => { return Err(FiatDepositError::File(err)) }
                }

                Ok(transaction)
            },
            Err(transaction_error) => Err(FiatDepositError::FiatTransactionError(transaction_error))
        }
    }

    // ➢ Retirar fiat por determinado medio: dado un monto de fiat se le descuenta dicho monto del balance
    // al usuario y se genera una transacción con la siguiente información:
    // fecha, usuario, tipo: retiro fiat, monto y medio (puede ser MercadoPago o Transferencia Bancaria)
    fn fiat_withdrawal(&mut self, today_date: Date, user_id: u32, amount: f64, mean: WithdrawalMean) -> Result<FiatTransaction, FiatWithdrawalError> {
        let data = CommonTransactionData {
            date: today_date,
            user: user_id,
            amount,
            transaction_type: TransactionType::FiatWithdrawal { mean },
        };

        match FiatTransaction::new(
            data,
        ) {
            Ok(transaction) => {
                // does user exist/have enough balance?
                if let Some(user) = self.users.get_mut(&data.user) {
                    // not enough! abort.
                    if user.fiat_balance < Balance(data.amount) { return Err( FiatWithdrawalError::NotEnoughBalance {
                        balance_needed: data.amount, balance: user.fiat_balance.f64()
                    }) }

                    // enough! substract balance
                    user.fiat_balance-= Balance(data.amount);
                } else {
                    return Err(FiatWithdrawalError::UserNotFound { user_id: data.user })
                }

                // guardar en archivo
                match self.sobreescribir_archivo_xyz() {
                    Ok(()) => {}
                    Err(err) => { return Err(FiatWithdrawalError::File(err)) }
                }

                Ok(transaction)
            }

            Err(error) => { Err(FiatWithdrawalError::FiatTransactionError( error )) }
        }
    }

    // ➢ Comprar determinada criptomoneda: dado un monto de fiat se compra una cantidad de determinada criptomoneda,
    //  tenga en cuenta que al momento de realizar la operación se obtiene del sistema
    //  la cotización actual de la criptomoneda para acreditar la correspondiente proporción en el balance
    //  de la cripto y desacreditar en el balance de fiat.
    // Luego de ello se registra la transacción con los siguientes datos:
    //      fecha, usuario, criptomoneda, tipo: compra de cripto, monto de cripto y cotización.
    fn crypto_buy(&mut self, today_date: Date, user_id: u32, fiat_amount: f64, crypto_prefix: &str)
                  -> Result<&CryptoTransaction, BuySellError> {
        // date errors are handled by CryptoTransaction::new()
        let data = CommonTransactionData {
            date: today_date,
            user: user_id,
            amount: fiat_amount,
            transaction_type: TransactionType::CryptoBuy,
        };

        match CryptoTransaction::new(
            data,
            crypto_prefix
        ) {
            Ok(transaction) => {
                // process buy

                // check 1: currency must be quoted AND quoted higher than 0.0
                let currency_unitary_value = if let Some(quoting) = self.quotes.get(crypto_prefix) {
                    if quoting.buy <= 0.0 { return Err(BuySellError::Unknown(format!("${crypto_prefix} is valued at {} FIAT (which is <= 0)", quoting.buy))) }
                    quoting.buy
                } else {
                    return Err(BuySellError::CryptocurrencyNotQuoted{ crypto_prefix: crypto_prefix.to_string() });
                };

                let transaction_crypto_amount = data.amount / currency_unitary_value; // .0 -> buy, .1 -> sell

                // check 2: user must exist
                if let Some(user) = self.users.get_mut(&data.user) {
                    // check 4: user must have enough fiat balance
                    if user.fiat_balance < Balance::from(data.amount) {
                        return Err(BuySellError::NotEnoughBalance{ balance: user.fiat_balance.f64(), balance_needed: data.amount })
                    }

                    // no error. execute operation
                    user.fiat_balance-= Balance::from(data.amount);
                    *user.crypto_balance.entry(crypto_prefix.to_string()).or_insert(Balance::from(0.0))+= Balance::from(transaction_crypto_amount);
                } else {
                    return Err(BuySellError::UserNotFound{ user_id: data.user });
                }

                self.crypto_transactions.entry(crypto_prefix.to_string()).or_default().push(transaction);

                // comprobar añadido y obtener préstamo
                match self.crypto_transactions.get(crypto_prefix) {
                    Some(crypto_transactions) => {
                        // la ultima transaccion guarada debería estar al final
                        match crypto_transactions.last() {
                            Some(transaction) => {
                                // todo bien!!!

                                // guardar en archivo
                                match self.sobreescribir_archivo_xyz() {
                                    Ok(()) => {}
                                    Err(err) => { return Err(BuySellError::File(err)) }
                                }

                                // to-do: desanidar este desastre (y similares). error repetido en todas las funciones.

                                Ok(transaction)
                            },
                            None => { Err(BuySellError::Unknown("El elemento añadido al historial no se encuentra en el historial.".to_string())) }
                        }
                    },
                    None => { Err(BuySellError::Unknown("El elemento añadido al historial no se encuentra en el historial.".to_string())) }
                }
            },
            Err(error) => Err(BuySellError::TransactionError(error))
        }
    }

    // ➢ Vender determinada criptomoneda: dado un monto de cripto se vende por fiat,
    //  tenga en cuenta que al momento de realizar la operación se obtiene del sistema la cotización actual
    //  de la criptomoneda para acreditar la correspondiente proporción en el balance de fiat
    //  y desacreditar en el balance de la criptomoneda.
    //  Luego de ello se registra la transacción con los siguientes datos:
    //  fecha, usuario, criptomoneda, tipo: venta de cripto, monto de cripto y cotización.
    fn crypto_sell(&mut self, today_date: Date, user_id: u32, crypto_amount: f64, crypto_prefix: &str) ->
    Result<&CryptoTransaction, BuySellError> {
        // date errors are handled by CryptoTransaction::new()
        let data = CommonTransactionData {
            date: today_date,
            user: user_id,
            amount: crypto_amount,
            transaction_type: TransactionType::CryptoSell,
        };

        match CryptoTransaction::new(
            data,
            crypto_prefix,
        ) {
            Ok(transaction) => {
                // process sell

                // check 1: currency must be quoted AND quoted higher than 0.0
                let currency_value = if let Some(quoting) = self.quotes.get(crypto_prefix) {
                    if quoting.sell <= 0.0 { return Err(BuySellError::Unknown(format!("${crypto_prefix} is valued at {} FIAT (which is <= 0)", quoting.sell))) }
                    quoting
                } else {
                    return Err(BuySellError::CryptocurrencyNotQuoted{ crypto_prefix: crypto_prefix.to_string() });
                };

                let transaction_fiat_value = currency_value.sell * data.amount;

                // check 2: user must exist
                if let Some(user) = self.users.get_mut(&data.user) {
                    // check 3: user must have enough $crypto_prefix balance
                    if let Some(user_crypto_balance) = user.crypto_balance.get_mut(crypto_prefix) {
                        if *user_crypto_balance < Balance::from(data.amount) {
                            return Err(BuySellError::NotEnoughBalance { balance: user_crypto_balance.f64(), balance_needed: data.amount })
                        }

                        // no error. execute operation
                        *user_crypto_balance-= Balance::from(data.amount);
                        user.fiat_balance+= Balance::from(transaction_fiat_value);
                    } else {
                        return Err(BuySellError::NotEnoughBalance{ balance: 0.0, balance_needed: data.amount })
                    }
                } else {
                    return Err(BuySellError::UserNotFound{ user_id: data.user });
                }

                self.crypto_transactions.entry(crypto_prefix.to_string()).or_default().push(transaction);

                // comprobar añadido y obtener préstamo
                match self.crypto_transactions.get(crypto_prefix) {
                    Some(crypto_transactions) => {
                        // la ultima transaccion guarada debería estar al final
                        match crypto_transactions.last() {
                            Some(transaction) => {
                                // todo bien!!!

                                // guardar en archivo
                                match self.sobreescribir_archivo_xyz() {
                                    Ok(()) => {}
                                    Err(err) => { return Err(BuySellError::File(err)) }
                                }

                                Ok(transaction)
                            },
                            None => { Err(BuySellError::Unknown("El elemento añadido al historial no se encuentra en el historial.".to_string())) }
                        }
                    },
                    None => { Err(BuySellError::Unknown("El elemento añadido al historial no se encuentra en el historial.".to_string())) }
                }
            },
            Err(error) => Err(BuySellError::TransactionError(error))
        }
    }

    // ➢ Recibir criptomoneda de blockchain: dado un monto de una cripto y una blockchain se le acredita
    // al balancede dicha cripto al usuario el monto. Luego se genera una transacción con los siguientes datos:
    // fecha, usuario, tipo: recepción cripto, blockchain, cripto, monto, cotización.
    fn blockchain_deposit(&mut self, today_date: Date, user_id: u32, amount: f64, blockchain: &str, crypto: &str) -> Result<BlockchainTransaction, BlockchainDepositError> {
        // does blockchain exist?
        if !self.blockchains.contains_key(blockchain) {
            return Err(BlockchainDepositError::BlockchainNotFound { blockchain: blockchain.to_string() })
        }

        // does crypto have a quote?
        let Some(quote) = self.quotes.get(crypto) else {
            return Err(BlockchainDepositError::CryptoNotQuoted { crypto: crypto.to_string() })
        };

        let data = CommonTransactionData {
            date: today_date,
            user: user_id,
            amount,
            transaction_type: TransactionType::BlockchainDeposit,
        };

        match BlockchainTransaction::new(
            data,
            blockchain,
            None,
            crypto,
            quote.clone() // quote should be cloned, as it changes over time. can't be copied due to containing f64
        ) {
            Ok(transaction) => {
                // add to balance
                // does user exist/have enough balance?
                if let Some(user) = self.users.get_mut(&data.user) {
                    // enough! ready to deposit
                    if let Some(crypto_balance) = user.crypto_balance.get_mut(crypto) {
                        *crypto_balance+= Balance::from(amount);
                    } else {
                        user.crypto_balance.insert(crypto.to_string(), Balance::from(amount));
                    }
                } else {
                    return Err(BlockchainDepositError::UserNotFound { user_id: data.user })
                }

                // guardar en archivo, finalizar
                match self.sobreescribir_archivo_xyz() {
                    Ok(()) => { Ok(transaction) }
                    Err(err) => { Err(BlockchainDepositError::File(err)) }
                }
            }
            Err(error) => { Err(BlockchainDepositError::BlockchainTransactionError( error )) }
        }
    }

    // ➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain
    // se le descuenta del balance de dicha cripto al usuario el monto,
    // la blockchain devuelve un hash que representa una transacción en ella
    // (esto hágalo retornando el nombre de la blockchain + un número random).
    // Luego se genera una transacción con los siguientes datos:
    // fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto, cotización.
    fn blockchain_withdrawal(&mut self, today_date: Date, user_id: u32, crypto_amount: f64, blockchain: &str, crypto: &str) -> Result<BlockchainTransaction, BlockchainWithdrawalError> {
        // does blockchain exist?
        if !self.blockchains.contains_key(blockchain) {
            return Err(BlockchainWithdrawalError::BlockchainNotFound { blockchain: blockchain.to_string() })
        }

        // does crypto have a quote?
        let Some(quote) = self.quotes.get(crypto) else {
            return Err(BlockchainWithdrawalError::CryptoNotQuoted { crypto: crypto.to_string() })
        };

        let data = CommonTransactionData {
            date: today_date,
            user: user_id,
            amount: crypto_amount,
            transaction_type: TransactionType::BlockchainWithdrawal,
        };

        match BlockchainTransaction::new(
            data,
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

                        if *balance >= Balance::from(crypto_amount) {
                            *balance-= Balance::from(data.amount);
                        } else {
                            return Err(BlockchainWithdrawalError::NotEnoughBalance { balance_needed: crypto_amount, balance: balance.f64() });
                        }

                    } else {
                        return Err(BlockchainWithdrawalError::NotEnoughBalance { balance: 0.0, balance_needed: data.amount } )
                    }
                } else {
                    return Err(BlockchainWithdrawalError::UserNotFound { user_id: data.user })
                }

                // guardar en archivo
                match self.sobreescribir_archivo_xyz() {
                    Ok(()) => {}
                    Err(err) => { return Err(BlockchainWithdrawalError::File(err)) }
                }

                Ok(transaction)
            }
            Err(error) => { Err(BlockchainWithdrawalError::BlockchainTransactionError( error )) }
        }
    }

    // ➢ Saber cual es la criptomoneda que más cantidad de ventas tiene
    fn most_times_sold_cryptocurrency(&self) -> Option<(&str, usize)> {
        let mut telemetry = ("", 0usize);

        for (prefix, transactions) in &self.crypto_transactions {
            let mut times_sold = 0usize;

            for transaction in transactions {
                if transaction.data.transaction_type == TransactionType::CryptoSell {
                    times_sold+= 1;
                }
            }

            if times_sold > telemetry.1 {
                telemetry.0 = prefix.as_str();
                telemetry.1 = times_sold;
            }
        }

        if telemetry.1 > 0 {
            Some(telemetry)
        } else {
            None
        }
    }

    // ➢ Saber cual es la criptomoneda que más cantidad de compras tiene
    fn most_times_bought_cryptocurrency(&self) -> Option<(&str, usize)> {
        let mut telemetry = ("", 0usize);

        for (prefix, transactions) in &self.crypto_transactions {
            let mut times_sold = 0usize;

            for transaction in transactions {
                if transaction.data.transaction_type == TransactionType::CryptoBuy {
                    times_sold+= 1;
                }
            }

            if times_sold > telemetry.1 {
                telemetry.0 = prefix.as_str();
                telemetry.1 = times_sold;
            }
        }

        if telemetry.1 > 0 {
            Some(telemetry)
        } else {
            None
        }
    }

    // ➢ Saber cual es la criptomoneda que más volumen de ventas tiene
    fn highest_sold_volume_cryptocurrency(&self) -> Option<(&str, f64)> {
        let mut telemetry = ("", 0.0f64);

        for (prefix, transactions) in &self.crypto_transactions {
            let mut amount = 0.0f64;

            for transaction in transactions {
                if transaction.data.transaction_type == TransactionType::CryptoSell {
                    amount+= transaction.data.amount;
                }
            }

            if amount > telemetry.1 {
                telemetry.0 = prefix.as_str();
                telemetry.1 = amount;
            }
        }

        if telemetry.1 > 0.0 {
            Some(telemetry)
        } else {
            None
        }
    }

    // ➢ Saber cual es la criptomoneda que más volumen de compras tiene
    fn highest_buy_volume_cryptocurrency(&self) -> Option<(&str, f64)> {
        let mut telemetry = ("", 0.0f64);

        for (prefix, transactions) in &self.crypto_transactions {
            let mut amount = 0.0f64;

            for transaction in transactions {
                if transaction.data.transaction_type == TransactionType::CryptoBuy {
                    amount+= transaction.data.amount;
                }
            }

            if amount > telemetry.1 {
                telemetry.0 = prefix.as_str();
                telemetry.1 = amount;
            }
        }

        if telemetry.1 > 0.0 {
            Some(telemetry)
        } else {
            None
        }
    }

    // Nota: Tanto para comprar. vender, retirar el usuario debe estar validado.
    // Se debe validar siempre que haya balance suficiente para realizar la operación
    // en los casos de compra, venta, retiro.
}

#[cfg(test)]
mod test_xyz {
    use std::fs;
    use super::*;

    fn delete_xyz_mock_json() -> bool {
        fs::remove_file(
            format!("{BASE_FOLDER}test_xyz.json")
        ).is_ok()
    }

    // fiat_balance: 13548.0
    // crypto balance: BTC 3.0, ETH 5000.0
    fn mock_user_0() -> User {
        User {
            first_name: "a".to_string(),
            last_name: "sd".to_string(),
            email: "asd@asd.asd".to_string(),
            id: 0,
            fiat_balance: Balance(13548.0),
            crypto_balance: HashMap::from([
                (String::from("BTC"), Balance::from(3.0)),
                (String::from("ETH"), Balance::from(5000.0))
            ]),
        }
    }

    // 3 crypto transactions each for buy/sell: BTC 1, BTC 2, ETH 5000
    // btc buy: $1000 sell: $900
    // eth buy: $10   sell: $9
    fn mock_test_xyz() -> XYZ {
        let mut xyz = XYZ::new(Some("test_xyz"));

        // 3 crypto transactions each for buy/sell: BTC 1, BTC 2, ETH 5000
        // sell:
        let trans1_sell = CryptoTransaction {
            data: CommonTransactionData {
                date: Date::default(),
                user: 0,
                amount: 1.0,
                transaction_type: TransactionType::CryptoSell,
            },
            currency: "BTC".to_string(),
        };
        let trans2_sell = CryptoTransaction {
            data: CommonTransactionData {
                date: Date::default(),
                user: 0,
                amount: 2.0,
                transaction_type: TransactionType::CryptoSell,
            },
            currency: "BTC".to_string(),
        };
        let trans3_sell = CryptoTransaction {
            data: CommonTransactionData {
                date: Date::default(),
                user: 0,
                amount: 5000.0,
                transaction_type: TransactionType::CryptoSell,
            },
            currency: "ETH".to_string(),
        };
        let trans4_sell = CryptoTransaction {
            data: CommonTransactionData {
                date: Date::default(),
                user: 0,
                amount: 1000.0,
                transaction_type: TransactionType::CryptoSell,
            },
            currency: "ETH".to_string(),
        };

        //
        // buy
        //

        let trans1_buy = CryptoTransaction {
            data: CommonTransactionData {
                date: Date::default(),
                user: 0,
                amount: 1.0,
                transaction_type: TransactionType::CryptoBuy,
            },
            currency: "BTC".to_string(),
        };
        let trans2_buy = CryptoTransaction {
            data: CommonTransactionData {
                date: Date::default(),
                user: 0,
                amount: 2.0,
                transaction_type: TransactionType::CryptoBuy,
            },
            currency: "BTC".to_string(),
        };
        let trans3_buy = CryptoTransaction {
            data: CommonTransactionData {
                date: Date::default(),
                user: 0,
                amount: 5000.0,
                transaction_type: TransactionType::CryptoBuy,
            },
            currency: "ETH".to_string(),
        };
        let trans4_buy = CryptoTransaction {
            data: CommonTransactionData {
                date: Date::default(),
                user: 0,
                amount: 1000.0,
                transaction_type: TransactionType::CryptoBuy,
            },
            currency: "ETH".to_string(),
        };

        let trans_vec_btc = vec![trans1_sell, trans2_sell, trans1_buy, trans2_buy];
        let trans_vec_eth = vec![trans3_sell, trans4_sell, trans3_buy, trans4_buy];
        xyz.crypto_transactions.insert(String::from("BTC"), trans_vec_btc);
        xyz.crypto_transactions.insert(String::from("ETH"), trans_vec_eth);

        //
        // quotes
        //

        xyz.quotes.insert(String::from("BTC"), Quote { buy: 1000.0, sell: 900.0 });
        xyz.quotes.insert(String::from("ETH"), Quote { buy: 10.0, sell: 9.0 });

        //
        // blockchains
        //

        let blockchain1 = Blockchain {
            name: "MARITO".to_string(),
            prefix: "MTO".to_string(),
            supported_cryptos: vec![String::from("BTC"), String::from("LIBRA")],
        };
        xyz.blockchains.insert(blockchain1.prefix.to_string(), blockchain1);

        //
        // user 0
        //

        xyz.users.insert(0, mock_user_0());

        xyz
    }

    #[test]
    fn test_new() {
        let xyz = XYZ {
            file_name: "test_xyz".to_string(),
            users: Users::default(),
            blockchains: Blockchains::default(),
            quotes: Quotes::default(),
            crypto_transactions: CryptoTransactionHistory::default()
        };

        assert_eq!(xyz, XYZ::new(Some("test_xyz")));
    }

    #[test]
    fn test_fiat_deposit() {
        delete_xyz_mock_json();
        let mut xyz = mock_test_xyz();

        let date_today = Date { day: 1, month: 1, year: 1 };

        //
        // good deposit
        //

        let fiat_deposit_id0 = xyz.fiat_deposit(date_today, 0, 10.0);
        let Ok(fiat_deposit_id0) = fiat_deposit_id0 else { panic!("Should be Ok"); };
        assert_eq!(fiat_deposit_id0.data.transaction_type, TransactionType::FiatDeposit, "Should be fiat deposit");

        // default fiat balance is 13548.0, should now be 13558.0
        let Some(user) = xyz.users.get(&0) else { panic!("User 0 should exist") };
        assert_eq!(user.fiat_balance, Balance::from(13558.0));

        //
        // deposit error: user not found
        //

        let fiat_deposit_id1 = xyz.fiat_deposit(date_today, 1, 10.0);
        assert_eq!(fiat_deposit_id1, Err(FiatDepositError::UserNotFound { user_id: 1 }));

        //
        // deposit error: transaction error (negative amount)
        //

        let fiat_deposit_id0 = xyz.fiat_deposit(date_today, 1, -10.0);
        assert_eq!(fiat_deposit_id0, Err(FiatDepositError::FiatTransactionError(ErrorNewTransaction::InvalidInputAmount { amount: -10.0 })));

        //
        // good deposit, file error
        //

        xyz.file_name = "/ASDASD/ASD/ASDAASD/AD/ASAD/".to_string();
        let fiat_deposit_id0 = xyz.fiat_deposit(date_today, 0, 10.0);
        assert_eq!(fiat_deposit_id0, Err(FiatDepositError::File(FileError::IO)));
    }

    #[test]
    fn test_fiat_withdrawal() {
        delete_xyz_mock_json();
        let mut xyz = mock_test_xyz();

        let date_today = Date { day: 1, month: 1, year: 1 };

        //
        // good withdrawal
        //

        let fiat_withdrawal_id0 = xyz.fiat_withdrawal(date_today, 0, 10.0, WithdrawalMean::MercadoPago);
        let Ok(fiat_withdrawal_id0) = fiat_withdrawal_id0 else { panic!("Should be Ok"); };
        assert_eq!(fiat_withdrawal_id0.data.transaction_type, TransactionType::FiatWithdrawal { mean: WithdrawalMean::MercadoPago }, "Should be fiat deposit");

        // default balance is 13548.0, should now be 13538.0
        let Some(user) = xyz.users.get(&0) else { panic!("User 0 should exist") };
        assert_eq!(user.fiat_balance, Balance::from(13538.0));

        //
        // withdrawal error: user not found
        //

        let fiat_withdrawal_id1 = xyz.fiat_withdrawal(date_today, 1, 10.0, WithdrawalMean::MercadoPago);
        assert_eq!(fiat_withdrawal_id1, Err(FiatWithdrawalError::UserNotFound { user_id: 1 }));

        //
        // withdrawal error: transaction error (negative amount)
        //

        let fiat_withdrawal_id0 = xyz.fiat_withdrawal(date_today, 0, -10.0, WithdrawalMean::MercadoPago);
        assert_eq!(fiat_withdrawal_id0, Err(FiatWithdrawalError::FiatTransactionError(ErrorNewTransaction::InvalidInputAmount { amount: -10.0 })));

        //
        // withdrawal error: not enough balance
        //

        let fiat_withdrawal_id0 = xyz.fiat_withdrawal(date_today, 0, 1_000_000.0, WithdrawalMean::BankTansfer);
        assert_eq!(fiat_withdrawal_id0, Err(FiatWithdrawalError::NotEnoughBalance { balance: 13538.0, balance_needed: 1_000_000.0 }));

        //
        // good withdrawal, file error
        //

        xyz.file_name = "/ASDASD/ASD/ASDAASD/AD/ASAD/".to_string();
        let fiat_withdrawal_id0 = xyz.fiat_withdrawal(date_today, 0, 10.0, WithdrawalMean::MercadoPago);
        assert_eq!(fiat_withdrawal_id0, Err(FiatWithdrawalError::File(FileError::IO)));
    }

    // xyz.quotes.insert(String::from("BTC"), Quote { buy: 1000.0, sell: 900.0 });
    // xyz.quotes.insert(String::from("ETH"), Quote { buy: 10.0, sell: 9.0 });

    #[test]
    fn test_crypto_buy() {
        delete_xyz_mock_json();
        let mut xyz = mock_test_xyz();

        let date_today = Date { day: 1, month: 1, year: 1 };

        //
        // good buy
        //

        let id0_buy1 = xyz.crypto_buy(date_today, 0, 2000.0, "BTC");
        let Ok(id0_buy1) = id0_buy1 else { panic!("Should be Ok"); };

        // default fiat balance is 13548. now it should be 11548. defalt BTC balance is 3. it should now be 5
        let Some(user) = xyz.users.get(&0) else { panic!("User 0 should exist") };
        assert_eq!(user.fiat_balance, Balance::from(11548.0)); // 13548 - 2000
        assert_eq!(user.crypto_balance.get("BTC"), Some(&Balance::from(5.0)));

        //
        // buy error: user does not exist
        //

        let id1_buy1 = xyz.crypto_buy(date_today, 1, 2000.0, "BTC");
        assert_eq!(id1_buy1, Err(BuySellError::UserNotFound { user_id: 1 }));

        //
        // buy error: currency not quoted
        //

        let id0_buy1 = xyz.crypto_buy(date_today, 0, 2.0, "LIBRA");
        assert_eq!(id0_buy1, Err(BuySellError::CryptocurrencyNotQuoted { crypto_prefix: "LIBRA".to_string() }));

        //
        // buy error: not enough balance
        //

        let id0_buy1 = xyz.crypto_buy(date_today, 0, 1_000_000.0, "BTC");
        assert_eq!(id0_buy1, Err(BuySellError::NotEnoughBalance { balance_needed: 1_000_000.0, balance: 11548.0 }));

        //
        // good buy, file error
        //

        xyz.file_name = "/ASDASD/ASD/ASDAASD/AD/ASAD/".to_string();
        let id0_buy1 = xyz.crypto_buy(date_today, 0, 2000.0, "BTC");
        assert_eq!(id0_buy1, Err(BuySellError::File(FileError::IO)));
    }

    #[test]
    fn test_crypto_sell() {
        delete_xyz_mock_json();
        let mut xyz = mock_test_xyz();

        let date_today = Date { day: 1, month: 1, year: 1 };

        //
        // good sell
        //

        let id0_sell1 = xyz.crypto_sell(date_today, 0, 2.0, "BTC");
        let Ok(id0_sell1) = id0_sell1 else { panic!("Should be Ok"); };

        // default fiat balance is 13548. now it should now be 15348. defalt BTC balance is 3. it should now be 1
        let Some(user) = xyz.users.get(&0) else { panic!("User 0 should exist") };
        assert_eq!(user.fiat_balance, Balance::from(15348.0)); // 13548 + 1800
        assert_eq!(user.crypto_balance.get("BTC"), Some(&Balance::from(1.0)));

        //
        // sell error: user does not exist
        //

        let id1_sell1 = xyz.crypto_sell(date_today, 1, 2000.0, "BTC");
        assert_eq!(id1_sell1, Err(BuySellError::UserNotFound { user_id: 1 }));

        //
        // sell error: currency not quoted
        //

        let id0_sell1 = xyz.crypto_sell(date_today, 0, 2.0, "LIBRA");
        assert_eq!(id0_sell1, Err(BuySellError::CryptocurrencyNotQuoted { crypto_prefix: "LIBRA".to_string() }));

        //
        // sell error: not enough balance
        //

        let id0_sell1 = xyz.crypto_sell(date_today, 0, 1_000_000.0, "BTC");
        assert_eq!(id0_sell1, Err(BuySellError::NotEnoughBalance { balance_needed: 1_000_000.0, balance: 1.0 }));

        //
        // sell error: transaction error
        //

        let id0_sell1 = xyz.crypto_sell(date_today, 0, -10.0, "BTC");
        assert_eq!(id0_sell1, Err(BuySellError::TransactionError(ErrorNewTransaction::InvalidInputAmount { amount: -10.0 })));

        //
        // good sell, file error
        //

        xyz.file_name = "/ASDASD/ASD/ASDAASD/AD/ASAD/".to_string();
        let id0_sell1 = xyz.crypto_sell(date_today, 0, 1.0, "BTC");
        assert_eq!(id0_sell1, Err(BuySellError::File(FileError::IO)));
    }

    #[test]
    fn test_blockchain_deposit() {
        delete_xyz_mock_json();
        let mut xyz = mock_test_xyz();

        let date_today = Date { day: 1, month: 1, year: 1 };

        //
        // good deposit
        //

        let id0 = xyz.blockchain_deposit(date_today, 0, 10.0, "MTO", "BTC");
        let Ok(id0) = id0 else { panic!("Should be Ok"); };
        // original btc balance is 3.0, should now be 13.0
        let Some(user) = xyz.users.get(&0) else { panic!("User 0 should exist"); };
        assert_eq!(user.crypto_balance.get("BTC"), Some(&Balance::from(13.0)));

        //
        // deposit error: blockchain not found
        //

        let id0 = xyz.blockchain_deposit(date_today, 0, 10.0, "ASD", "BTC");
        assert_eq!(id0, Err(BlockchainDepositError::BlockchainNotFound { blockchain: "ASD".to_string() }));

        //
        // deposit error: crypto not quoted
        //

        let id0 = xyz.blockchain_deposit(date_today, 0, 10.0, "MTO", "LIBRA");
        assert_eq!(id0, Err(BlockchainDepositError::CryptoNotQuoted { crypto: "LIBRA".to_string() }));

        //
        // deposit error: user not found
        //

        let id1 = xyz.blockchain_deposit(date_today, 1, 10.0, "MTO", "BTC");
        assert_eq!(id1, Err(BlockchainDepositError::UserNotFound { user_id: 1 }));

        //
        // deposit error: transaction error
        //

        let id0 = xyz.blockchain_deposit(date_today, 0, -10.0, "MTO", "BTC");
        assert_eq!(id0, Err(BlockchainDepositError::BlockchainTransactionError(ErrorNewTransaction::InvalidInputAmount { amount: -10.0 })));

        //
        // good deposit, file error
        //

        xyz.file_name = "/ASDASD/ASD/ASDAASD/AD/ASAD/".to_string();
        let id0 = xyz.blockchain_deposit(date_today, 0, 10.0, "MTO", "BTC");
        assert_eq!(id0, Err(BlockchainDepositError::File(FileError::IO)));
    }

    #[test]
    fn test_blockchain_withdrawal() {
        delete_xyz_mock_json();
        let mut xyz = mock_test_xyz();

        let date_today = Date { day: 1, month: 1, year: 1 };

        //
        // good withdrawal
        //

        let id0 = xyz.blockchain_withdrawal(date_today, 0, 1.0, "MTO", "BTC");
        println!("{id0:?}");
        let Ok(id0) = id0 else { panic!("Should be Ok"); };
        // original btc balance is 3.0, should now be 2.0
        let Some(user) = xyz.users.get(&0) else { panic!("User 0 should exist"); };
        assert_eq!(user.crypto_balance.get("BTC"), Some(&Balance::from(2.0)));

        //
        // withdrawal error: not enough balance
        //

        let id0 = xyz.blockchain_withdrawal(date_today, 0, 100.0, "MTO", "BTC");
        assert_eq!(id0, Err(BlockchainWithdrawalError::NotEnoughBalance { balance_needed: 100.0, balance: 2.0 }));

        //
        // withdrawal error: blockchain not found
        //

        let id0 = xyz.blockchain_withdrawal(date_today, 0, 1.0, "ASD", "BTC");
        assert_eq!(id0, Err(BlockchainWithdrawalError::BlockchainNotFound { blockchain: "ASD".to_string() }));

        //
        // withdrawal error: crypto not quoted
        //

        let id0 = xyz.blockchain_withdrawal(date_today, 0, 1.0, "MTO", "LIBRA");
        assert_eq!(id0, Err(BlockchainWithdrawalError::CryptoNotQuoted { crypto: "LIBRA".to_string() }));

        //
        // withdrawal error: user not found
        //

        let id1 = xyz.blockchain_withdrawal(date_today, 1, 10.0, "MTO", "BTC");
        assert_eq!(id1, Err(BlockchainWithdrawalError::UserNotFound { user_id: 1 }));

        //
        // withdrawal error: transaction error
        //

        let id0 = xyz.blockchain_withdrawal(date_today, 0, -10.0, "MTO", "BTC");
        assert_eq!(id0, Err(BlockchainWithdrawalError::BlockchainTransactionError(ErrorNewTransaction::InvalidInputAmount { amount: -10.0 })));

        //
        // good withdrawal, file error
        //

        xyz.file_name = "/ASDASD/ASD/ASDAASD/AD/ASAD/".to_string();
        let id0 = xyz.blockchain_withdrawal(date_today, 0, 1.0, "MTO", "BTC");
        assert_eq!(id0, Err(BlockchainWithdrawalError::File(FileError::IO)));
    }

    //
    // 4 crypto transactions each for buy/sell: BTC 1, BTC 2, ETH 5000, ETH 1000
    //

    #[test]
    fn test_most_times_sold_cryptocurrency() {
        delete_xyz_mock_json();
        let xyz = mock_test_xyz();

        let data = xyz.most_times_sold_cryptocurrency();
        assert!(data.is_some(), "Should be Some");
        let data = data.unwrap();

        assert_eq!(data.0, "BTC");
        assert_eq!(data.1, 2usize);
    }

    #[test]
    fn test_highest_sold_volume_cryptocurrency() {
        delete_xyz_mock_json();
        let xyz = mock_test_xyz();

        let data = xyz.highest_sold_volume_cryptocurrency();
        assert!(data.is_some(), "Should be Some");
        let data = data.unwrap();

        assert_eq!(data.0, "ETH");
        assert_eq!(data.1.as_balance(), 6000.0.as_balance());
    }

    #[test]
    fn test_most_times_bought_cryptocurrency() {
        delete_xyz_mock_json();
        let xyz = mock_test_xyz();

        let data = xyz.most_times_bought_cryptocurrency();
        assert!(data.is_some(), "Should be Some");
        let data = data.unwrap();

        assert_eq!(data.0, "BTC");
        assert_eq!(data.1, 2usize);
    }

    #[test]
    fn test_highest_buy_volume_cryptocurrency() {
        delete_xyz_mock_json();
        let xyz = mock_test_xyz();

        let data = xyz.highest_buy_volume_cryptocurrency();
        assert!(data.is_some(), "Should be Some");
        let data = data.unwrap();

        assert_eq!(data.0, "ETH");
        assert_eq!(data.1.as_balance(), 6000.0.as_balance());
    }
}