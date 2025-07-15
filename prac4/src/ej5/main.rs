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
//mod structs;

//
// date.rs
//

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::ops::{AddAssign, SubAssign};

const NOMBRE_MESES: [&str; 12] = ["Enero", "Febrero", "Marzo", "Abril",
    "Mayo", "Junio", "Julio", "Agosto",
    "Septiembre", "Octubre", "Noviembre", "Diciembre"];
#[derive(Clone, PartialEq, Debug, Copy, Hash)]
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
            let dias_para_proximo_mes = (dias_mes_actual - self.day + 1) as u32;

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
                self.day += dias_restantes as u8;
                dias_restantes = 0;
            }
        }
    }

    pub fn subtract_days(&mut self, dias: u32) {
        let mut dias_restantes = dias;

        while dias_restantes > 0 {
            if dias_restantes >= self.day as u32 {
                // ir al anterior mes
                dias_restantes-= self.day as u32;
                self.month -= 1;

                if self.month < 1 {
                    self.month = 12;
                    self.year -= 1;
                }

                // corregir self.dia == 0
                self.day = self.current_month_days();
            } else {
                self.day -= dias_restantes as u8;
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

//
// user.rs
//

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
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
    pub fn f64(&self) -> f64 { self.0 }
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

trait AsBalance {
    fn as_balance(&self) -> Balance;
}
impl AsBalance for f64 {
    fn as_balance(&self) -> Balance {
        Balance(self.clone())
    }
}

// user

#[derive(Debug)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub dni: u32, // primary key
    pub identity_validation: bool,
    pub fiat_balance: Balance,
    pub crypto_balance: HashMap<String, Balance>
}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first_name.hash(state);
        self.last_name.hash(state);
        self.email.hash(state);
        self.dni.hash(state);
        self.identity_validation.hash(state);
        self.fiat_balance.hash(state);
    }
}

//
// monetary_structs.rs
//

#[derive(Debug)]
pub struct BlockchainTransactionHash(String);
impl BlockchainTransactionHash {
    fn new(prefix: &str) -> BlockchainTransactionHash {
        BlockchainTransactionHash(format!("{}-{}", prefix, rand::random::<u32>()))
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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
#[derive(Clone, Debug)]
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

#[derive(Debug)]
#[derive(PartialEq)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

//
// xyz.rs
//

pub struct XYZ {
    pub users: BTreeMap<u32, User>,
    pub blockchains: BTreeMap<String, Blockchain>,
    pub quotes: HashMap<String, Quote> // (buy, sell) prices
}

//
// errors
//

#[derive(Debug)]
pub enum ErrorFiatDeposit {
    FiatTransactionError(ErrorNewTransaction),
    UserNotFound{ user_dni: u32 }
}

#[derive(Debug)]
pub enum ErrorFiatWithdraw {
    FiatTransactionError(ErrorNewTransaction),
    UserNotFound{ user_dni: u32 },
    NotEnoughBalance{ balance: f64, balance_needed: f64 },
}

#[derive(Debug)]
pub enum ErrorBlockchainDeposit {
    BlockchainTransactionError(ErrorNewTransaction),
    BlockchainNotFound{ blockchain: String },
    CryptoNotQuoted{ crypto: String },
    UserNotFound{ user_dni: u32 },
}

#[derive(Debug)]
pub enum ErrorBlockchainWithdraw {
    BlockchainTransactionError(ErrorNewTransaction),
    BlockchainNotFound{ blockchain: String },
    CryptoNotQuoted{ crypto: String },
    UserNotFound{ user_dni: u32 },
    NotEnoughBalance{ balance: f64, balance_needed: f64 }
}

#[derive(Debug)]
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
    fn fiat_deposit(&mut self, date: Date, user: u32, fiat_amount: f64) -> Result<FiatTransaction, ErrorFiatDeposit> {
        let data = CommonTransactionData { date, user, amount: fiat_amount };

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
    fn fiat_withdraw(&mut self, date: Date, user: u32, fiat_amount: f64, mean: WithdrawalMean) -> Result<FiatTransaction, ErrorFiatWithdraw> {
        let data = CommonTransactionData { date, user, amount: fiat_amount };

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
    fn buy_crypto(&mut self, date: Date, user: u32, fiat_amount: f64, crypto_prefix: &str)
                  -> Result<CryptoTransaction, ErrorBuySell> {
        // date errors are handled by CryptoTransaction::new()

        // check 1: invalid fiat amount
        if fiat_amount < 0.0 {
            return Err(ErrorBuySell::NegativeAmount)
        }

        let data = CommonTransactionData { date, user, amount: fiat_amount };

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
                    return Err(ErrorBuySell::CryptocurrencyNotQuoted { crypto_prefix: crypto_prefix.to_string() });
                };

                let transaction_crypto_amount = fiat_amount / currency_unitary_value; // .0 -> buy, .1 -> sell

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
    fn sell_crypto(&mut self, date: Date, user: u32, crypto_amount: f64, crypto_prefix: &str) ->
    Result<CryptoTransaction, ErrorBuySell> {
        // date errors are handled by CryptoTransaction::new()

        // check 1: amounts should be higher than 0
        if crypto_amount <= 0.0 {
            return Err(ErrorBuySell::NegativeAmount)
        }

        let data = CommonTransactionData { date, user, amount: crypto_amount };

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

                let transaction_fiat_value = currency_value.sell * crypto_amount;

                // check 3: user must exist
                if let Some(user) = self.users.get_mut(&data.user) {
                    // check 4: user must have enough $crypto_prefix balance
                    if let Some(user_crypto_balance) = user.crypto_balance.get_mut(crypto_prefix) {
                        if *user_crypto_balance < Balance::from(crypto_amount) {
                            return Err(ErrorBuySell::NotEnoughBalance { balance: user_crypto_balance.f64(), balance_needed: crypto_amount })
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
    fn withdraw_to_blockchain(&mut self, date: Date, user: u32, crypto_amount: f64, blockchain: &str, crypto: &str) -> Result<BlockchainTransaction, ErrorBlockchainWithdraw> {
        // does blockchain exist?
        if !self.blockchains.contains_key(blockchain) {
            return Err(ErrorBlockchainWithdraw::BlockchainNotFound { blockchain: blockchain.to_string() })
        };

        let data = CommonTransactionData { date, user, amount: crypto_amount };

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
                        if balance < &mut Balance::from(crypto_amount) {
                            return Err(ErrorBlockchainWithdraw::NotEnoughBalance { balance: balance.f64(), balance_needed: data.amount } )
                        }
                        *balance-= Balance::from(crypto_amount);
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
    fn deposit_from_blockchain(&mut self, date: Date, user: u32, crypto_amount: f64, blockchain: &str, crypto: &str) -> Result<BlockchainTransaction, ErrorBlockchainDeposit> {
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

        let data = CommonTransactionData { date, user, amount: crypto_amount };

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
                    // enough! ready to deposit
                    *user.crypto_balance.entry(crypto.to_string()).or_insert(Balance::from(0.0))+= Balance::from(crypto_amount);
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

fn main() {}

#[cfg(test)]
mod tests {
    use std::hash::DefaultHasher;
    use super::*;

    #[test]
    fn test_date() {
        let date = Date::new(1, 1, 2023).unwrap();
        assert_eq!(date.to_string(), "1 de Enero del 2023");
        assert!(date.is_date_valid());
        assert!(!Date::new(31, 2, 2023).is_some());
        
        let mut date = Date::new(28, 2, 2020).unwrap();
        assert!(date.is_leap_year());
        date.add_days(1);
        assert_eq!(date.to_string(), "29 de Febrero del 2020");
        date.add_days(1);
        assert_eq!(date.to_string(), "1 de Marzo del 2020");
        date.subtract_days(2);
        assert_eq!(date.to_string(), "28 de Febrero del 2020");
        date.subtract_days(28);
        assert_eq!(date.to_string(), "31 de Enero del 2020");
        
        // test partialcmp
        let date1 = Date::new(1, 1, 2023).unwrap();
        let date2 = Date::new(2, 1, 2023).unwrap();
        
        assert!(date1 < date2);
        assert!(date2 > date1);
        
        let date3 = Date::new(1, 1, 2023).unwrap();
        assert!(date1 == date3);
    }

    #[test]
    fn test_user_hash() {
        let user1 = User {
            first_name: "Alice".to_string(),
            last_name: "Smith".to_string(),
            email: "asd@asd.asd".to_string(),
            dni: 12345678,
            identity_validation: true,
            fiat_balance: Balance::new(1000.0),
            crypto_balance: HashMap::new()
        };
        
        let user2 = User {
            first_name: "Alice".to_string(),
            last_name: "Smith".to_string(),
            email: "asd@asd.asd".to_string(),
            dni: 12345678,
            identity_validation: true,
            fiat_balance: Balance::new(1000.0),
            crypto_balance: HashMap::new()
        };
        
        let mut hasher1 = DefaultHasher::new();
        user1.hash(&mut hasher1);
        
        let mut hasher2 = DefaultHasher::new();
        user2.hash(&mut hasher2);
        
        assert_eq!(hasher1.finish(), hasher2.finish());
    }
    
    #[test]
    fn test_balance() {
        let mut balance = Balance::new(100.0);
        balance.add_assign_f64(50.0);
        assert_eq!(balance.f64(), 150.0);
        balance.sub_assign_f64(30.0);
        assert_eq!(balance.f64(), 120.0);
    }

    #[test]
    fn test_user() {
        let mut user = User {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "asd@asd.asd".to_string(),
            dni: 12345678,
            identity_validation: true,
            fiat_balance: Balance::new(1000.0),
            crypto_balance: HashMap::new()
        };

        user.fiat_balance.add_assign_f64(500.0);
        assert_eq!(user.fiat_balance.f64(), 1500.0);

        user.fiat_balance.sub_assign_f64(200.0);
        assert_eq!(user.fiat_balance.f64(), 1300.0);

        user.crypto_balance.insert("BTC".to_string(), Balance::new(0.5));
        if let Some(balance) = user.crypto_balance.get_mut("BTC") {
            balance.add_assign_f64(0.1);
        } else {
            panic!("BTC balance not found");
        }
        assert_eq!(user.crypto_balance.get("BTC").unwrap().f64(), 0.6);

        if let Some(balance) = user.crypto_balance.get_mut("ETH") {
            balance.add_assign_f64(1.0);
        } else {
            user.crypto_balance.insert("ETH".to_string(), Balance::new(1.0));
        }
        assert_eq!(user.crypto_balance.get("ETH").unwrap().f64(), 1.0);
    }

    #[test]
    fn test_fiat_transaction() {
        let data = CommonTransactionData {
            date: Date::new(1, 1, 2023).unwrap(),
            user: 12345678,
            amount: 100.0
        };

        let transaction = FiatTransaction::new(data, TransactionType::FiatDeposit);
        assert!(transaction.is_ok());

        let transaction = transaction.unwrap();
        assert_eq!(transaction.data.date.to_string(), "1 de Enero del 2023");

        assert_eq!(transaction.data.user, 12345678);
        assert_eq!(transaction.data.amount, 100.0);
        assert_eq!(transaction.transaction_type, TransactionType::FiatDeposit);
        assert!(transaction.data.date.is_date_valid());
    }

    #[test]
    fn test_crypto_new() {
        // test Cryptocurrency::new()
        let crypto = Cryptocurrency::new("Bitcoin", "BTC", vec!["BTC".to_string(), "ETH".to_string()]);
        assert!(crypto.is_ok());
        
        let crypto = crypto.unwrap();
        assert_eq!(crypto.name, "Bitcoin");
        assert_eq!(crypto.prefix, "BTC");
        assert_eq!(crypto.blockchains.len(), 2);
        assert!(crypto.blockchains.contains(&"BTC".to_string()));
        assert!(crypto.blockchains.contains(&"ETH".to_string()));
    }
    
    #[test]
    fn test_transactiontype_display() {
        let fiat_deposit = TransactionType::FiatDeposit;
        let fiat_withdraw = TransactionType::FiatWithdrawal { mean: WithdrawalMean::BankTansfer };
        let blockchain_deposit = TransactionType::BlockchainDeposit;
        let blockchain_withdrawal = TransactionType::BlockchainWithdrawal;
        let crypto_buy = TransactionType::CryptoBuy;
        let crypto_sell = TransactionType::CryptoSell;

        assert_eq!(fiat_deposit.to_string(), "Fiat Deposit");
        assert_eq!(fiat_withdraw.to_string(), "Fiat Withdrawal via BankTansfer");
        assert_eq!(blockchain_deposit.to_string(), "Blockchain Deposit");
        assert_eq!(blockchain_withdrawal.to_string(), "Blockchain Withdrawal");
        assert_eq!(crypto_buy.to_string(), "Crypto Buy");
        assert_eq!(crypto_sell.to_string(), "Crypto Sell");
    }
    
    #[test]
    fn test_crypto_transaction() {
        let data = CommonTransactionData {
            date: Date::new(1, 1, 2023).unwrap(),
            user: 12345678,
            amount: 0.5
        };

        let transaction = CryptoTransaction::new(data, TransactionType::CryptoBuy, "BTC");
        assert!(transaction.is_ok());

        let transaction = transaction.unwrap();
        assert_eq!(transaction.data.date.to_string(), "1 de Enero del 2023");

        assert_eq!(transaction.data.user, 12345678);
        assert_eq!(transaction.data.amount, 0.5);
        assert_eq!(transaction.currency, "BTC");
        assert!(transaction.data.date.is_date_valid());
    }

    #[test]
    fn test_blockchain_transaction() {
        let data = CommonTransactionData {
            date: Date::new(1, 1, 2023).unwrap(),
            user: 12345678,
            amount: 0.5
        };

        let blockchain = Blockchain::new("Bitcoin", "BTC", vec!["BTC".to_string()]);
        let transaction = blockchain.withdraw(data, TransactionType::BlockchainWithdrawal, "BTC", Quote { buy: 50000.0, sell: 49000.0 });
        assert!(transaction.is_ok());
        let transaction = transaction.unwrap();

        assert_eq!(transaction.data.date.to_string(), "1 de Enero del 2023");
        assert_eq!(transaction.data.user, 12345678);
        assert_eq!(transaction.data.amount, 0.5);
        assert_eq!(transaction.blockchain, "Bitcoin");
        assert_eq!(transaction.crypto, "BTC");
        assert_eq!(transaction.quote.buy, 50000.0);
        assert_eq!(transaction.quote.sell, 49000.0);
        assert!(transaction.data.date.is_date_valid());
    }

    #[test]
    fn test_xyz_fiat() {
        let mut xyz = XYZ::new(
            BTreeMap::new(),
            BTreeMap::new(),
            HashMap::new()
        );

        let user = User {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "asd@asd.asd".to_string(),
            dni: 12345678,
            identity_validation: true,
            fiat_balance: Balance::new(1000.0),
            crypto_balance: HashMap::new()
        };

        xyz.users.insert(user.dni, user);
        let data = CommonTransactionData {
            date: Date::new(1, 1, 2023).unwrap(),
            user: 12345678,
            amount: 100.0
        };

        let transaction = xyz.fiat_deposit(data.date, data.user, data.amount);
        assert!(transaction.is_ok());
        let transaction = transaction.unwrap();
        assert_eq!(transaction.data.date.to_string(), "1 de Enero del 2023");
        assert_eq!(transaction.data.user, 12345678);
        assert_eq!(transaction.data.amount, 100.0);
        assert_eq!(transaction.transaction_type, TransactionType::FiatDeposit);
        assert!(transaction.data.date.is_date_valid());

        let user = xyz.users.get(&12345678).unwrap();
        assert_eq!(user.fiat_balance.f64(), 1100.0); // 1000 + 100 deposit

        let transaction = xyz.fiat_withdraw(data.date, data.user, data.amount, WithdrawalMean::BankTansfer);
        assert!(transaction.is_ok());

        let transaction = transaction.unwrap();
        assert_eq!(transaction.data.date.to_string(), "1 de Enero del 2023");
        assert_eq!(transaction.data.user, 12345678);
        assert_eq!(transaction.data.amount, 100.0);
        assert_eq!(transaction.transaction_type, TransactionType::FiatWithdrawal { mean: WithdrawalMean::BankTansfer });
        assert!(transaction.data.date.is_date_valid());

        let user = xyz.users.get(&12345678).unwrap();
        assert_eq!(user.fiat_balance.f64(), 1000.0); // 1100 - 100 withdraw
    }

    #[test]
    fn test_xyz_crypto() {
        let mut xyz = XYZ::new(
            BTreeMap::new(),
            BTreeMap::new(),
            HashMap::new()
        );

        let user = User {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "asd@asd.asd".to_string(),
            dni: 12345678,
            identity_validation: true,
            fiat_balance: Balance::new(1000.0),
            crypto_balance: HashMap::new()
        };

        xyz.users.insert(user.dni, user);
        xyz.quotes.insert("BTC".to_string(), Quote { buy: 50000.0, sell: 49000.0 });

        let data = CommonTransactionData {
            date: Date::new(1, 1, 2023).unwrap(),
            user: 12345678,
            amount: 1000.0 // fiat amount
        };

        // buy crypto
        let transaction = xyz.buy_crypto(data.date, data.user, data.amount, "BTC");
        assert!(transaction.is_ok());
        let transaction = transaction.unwrap();
        assert_eq!(transaction.data.date.to_string(), "1 de Enero del 2023");
        assert_eq!(transaction.data.user, 12345678);
        assert_eq!(transaction.data.amount, 1000.0);
        assert_eq!(transaction.currency, "BTC");
        assert_eq!(transaction.data.date.is_date_valid(), true);

        let user = xyz.users.get(&12345678).unwrap();
        assert_eq!(user.fiat_balance.f64(), 0.0); // 1000 - 100 buy
        assert_eq!(user.crypto_balance.get("BTC").unwrap().f64(), 0.02); // 100 / 50000 = 0.002 BTC

        // sell crypto
        let transaction = xyz.sell_crypto(data.date, data.user, 0.02, "BTC");
        assert!(transaction.is_ok());
        let transaction = transaction.unwrap();
        assert_eq!(transaction.data.date.to_string(), "1 de Enero del 2023");
        assert_eq!(transaction.data.user, 12345678);
        assert_eq!(transaction.data.amount, 0.02); // amount in crypto
        assert_eq!(transaction.currency, "BTC");
        assert_eq!(transaction.data.date.is_date_valid(), true);

        let user = xyz.users.get(&12345678).unwrap();
        assert_eq!(user.fiat_balance.f64(), 0.02 * 49000.0); // 900 + (0.002 * 49000) = 900 + 98 = 998
        assert_eq!(user.crypto_balance.get("BTC").unwrap().f64(), 0.0); // all BTC sold
    }

    #[test]
    fn test_xyz_blockchain() {
        let mut xyz = XYZ::new(
            BTreeMap::new(),
            BTreeMap::new(),
            HashMap::new()
        );

        let mut user = User {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "asd@asd.asd".to_string(),
            dni: 12345678,
            identity_validation: true,
            fiat_balance: Balance::new(1000.0),
            crypto_balance: HashMap::new()
        };

        // add crypto balance
        user.crypto_balance.insert("BTC".to_string(), Balance::new(0.02)); // 0.02 BTC

        xyz.users.insert(user.dni, user);
        xyz.quotes.insert("BTC".to_string(), Quote { buy: 50000.0, sell: 49000.0 });
        xyz.blockchains.insert("Bitcoin".to_string(), Blockchain::new("Bitcoin", "BTC", vec!["BTC".to_string()]));

        let data = CommonTransactionData {
            date: Date::new(1, 1, 2023).unwrap(),
            user: 12345678,
            amount: 0.02 // crypto amount
        };

        // success: withdraw to blockchain
        let transaction = xyz.withdraw_to_blockchain(data.date, data.user, data.amount, "Bitcoin", "BTC");
        assert!(transaction.is_ok());

        let transaction = transaction.unwrap();
        assert_eq!(transaction.data.date.to_string(), "1 de Enero del 2023");
        assert_eq!(transaction.data.user, 12345678);
        assert_eq!(transaction.data.amount, 0.02);
        assert_eq!(transaction.blockchain, "Bitcoin");
        assert_eq!(transaction.crypto, "BTC");
        assert!(transaction.data.date.is_date_valid());

        let user = xyz.users.get(&12345678).unwrap();
        assert_eq!(user.crypto_balance.get("BTC").unwrap().f64(), 0.0); // all BTC withdrawn
        // hash does not need testing

        // errors
        // error: withdraw to non-existing blockchain
        let transaction = xyz.withdraw_to_blockchain(data.date, data.user, data.amount, "NonExistingBlockchain", "BTC");
        assert!(transaction.is_err());
        if let Err(ErrorBlockchainWithdraw::BlockchainNotFound { blockchain }) = transaction {
            assert_eq!(blockchain, "NonExistingBlockchain");
        } else {
            panic!("Expected BlockchainNotFound error");
        }

        // error: withdraw to blockchain with non-existing crypto
        let transaction = xyz.withdraw_to_blockchain(data.date, data.user, data.amount, "Bitcoin", "ETH");
        assert!(transaction.is_err());
        if let Err(ErrorBlockchainWithdraw::CryptoNotQuoted { crypto }) = transaction {
            assert_eq!(crypto, "ETH");
        } else {
            panic!("Expected CryptoNotQuoted error");
        }

        // error: withdraw to blockchain with non-existing user
        let transaction = xyz.withdraw_to_blockchain(data.date, 99999999, data.amount, "Bitcoin", "BTC");
        assert!(transaction.is_err());
        if let Err(ErrorBlockchainWithdraw::UserNotFound { user_dni }) = transaction {
            assert_eq!(user_dni, 99999999);
        } else {
            panic!("Expected UserNotFound error");
        }

        // error: withdraw to blockchain with not enough balance
        let transaction = xyz.withdraw_to_blockchain(data.date, data.user, 1000.0, "Bitcoin", "BTC");
        assert!(transaction.is_err());
        if let Err(ErrorBlockchainWithdraw::NotEnoughBalance { balance, balance_needed }) = transaction {
            assert_eq!(balance, 0.0);
            assert_eq!(balance_needed, 1000.0);
        } else {
            panic!("Expected NotEnoughBalance error");
        }
        
        // success: deposit from blockchain
        let transaction = xyz.deposit_from_blockchain(data.date, data.user, 0.02, "Bitcoin", "BTC");
        assert!(transaction.is_ok());
        let transaction = transaction.unwrap();
        assert_eq!(transaction.data.date.to_string(), "1 de Enero del 2023");
        assert_eq!(transaction.data.user, 12345678);
        assert_eq!(transaction.data.amount, 0.02);
        assert_eq!(transaction.blockchain, "Bitcoin");
        assert_eq!(transaction.crypto, "BTC");
        assert!(transaction.data.date.is_date_valid());
        
        let user = xyz.users.get(&12345678).unwrap();
        assert_eq!(user.crypto_balance.get("BTC").unwrap().f64(), 0.02); // 0.02 BTC deposited
        
        // error: deposit from non-existing blockchain
        let transaction = xyz.deposit_from_blockchain(data.date, data.user, data.amount, "NonExistingBlockchain", "BTC");
        assert!(transaction.is_err());
        if let Err(ErrorBlockchainDeposit::BlockchainNotFound { blockchain }) = transaction {
            assert_eq!(blockchain, "NonExistingBlockchain");
        } else {
            panic!("Expected BlockchainNotFound error");
        }
        
        // error: deposit from blockchain with non-existing crypto
        let transaction = xyz.deposit_from_blockchain(data.date, data.user, data.amount, "Bitcoin", "ETH");
        assert!(transaction.is_err());
        if let Err(ErrorBlockchainDeposit::CryptoNotQuoted { crypto }) = transaction {
            assert_eq!(crypto, "ETH");
        } else {
            panic!("Expected CryptoNotQuoted error");
        }
        
        // error: deposit from blockchain with non-existing user
        let transaction = xyz.deposit_from_blockchain(data.date, 99999999, data.amount, "Bitcoin", "BTC");
        assert!(transaction.is_err());
        if let Err(ErrorBlockchainDeposit::UserNotFound { user_dni }) = transaction {
            assert_eq!(user_dni, 99999999);
        } else {
            panic!("Expected UserNotFound error");
        }
    }
}