// this is the struct for representing market data in the stream.
use chrono::Utc;
use serde::{Deserialize, Deserializer, Serialize};
use serde_aux::prelude::*;
use std::string;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ExchangeName {
    Binance = 2,
    CoinBase = 3,
}

impl ExchangeName {
    pub const COUNT: usize = 2;

    pub fn all() -> Vec<ExchangeName> {
        vec![ExchangeName::Binance, ExchangeName::CoinBase]
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum Token {
    BTCUSDT = 5,
    UNKOWN = 7,
    ETHUSDT = 11,
    BNBUSDT = 13,
    TRBUSDT = 15,
    LINKUSDT = 17,
    ADAUSDT = 19,
}

impl Token {
    pub fn all() -> Vec<Token> {
        vec![
            Token::BTCUSDT,
            Token::UNKOWN,
            Token::ETHUSDT,
            Token::BNBUSDT,
            Token::TRBUSDT,
            Token::LINKUSDT,
            Token::ADAUSDT,
        ]
    }
}

impl Token {
    fn asToken(symbol: String) -> Token {
        match symbol.as_str() {
            "BTCUSDT" => Token::BTCUSDT,
            "BTC-USDT" => Token::BTCUSDT,
            "ETHUSDT" => Token::ETHUSDT,
            "ETH-USDT" => Token::ETHUSDT,
            "TRBUSDT" => Token::TRBUSDT,
            "TRB-USDT" => Token::TRBUSDT,
            "BNBUSDT" => Token::BNBUSDT,
            "BNB-USDT" => Token::BNBUSDT,
            "ADAUSDT" => Token::ADAUSDT,
            "ADA-USDT" => Token::ADAUSDT,
            "LINKUSDT" => Token::LINKUSDT,
            "LINK-USDT" => Token::LINKUSDT,
            _ => Token::UNKOWN,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AskMarketQuote {
    pub exchange: ExchangeName,
    pub token: Token,
    pub price: f64,
    pub quantity: f64,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone)]
pub struct BidMarketQuote {
    pub exchange: ExchangeName,
    pub token: Token,
    pub price: f64,
    pub quantity: f64,
    pub timestamp_ms: u64,
}
// string to f64 converter for serde:
fn deserialize_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

// intermediary structs

//Binance
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BinanceBookTicker {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "a", deserialize_with = "deserialize_string_to_f64")]
    pub askprice: f64,
    #[serde(rename = "b", deserialize_with = "deserialize_string_to_f64")]
    pub bidprice: f64,
    #[serde(rename = "A", deserialize_with = "deserialize_string_to_f64")]
    pub askquantity: f64,
    #[serde(rename = "B", deserialize_with = "deserialize_string_to_f64")]
    pub bidquantity: f64,
}

impl BinanceBookTicker {
    pub fn getAskMarketQuotes(&self) -> Vec<AskMarketQuote> {
        let mut AskMarketQuoteVec: Vec<AskMarketQuote> = Vec::new();

        let timestamp_millis = Utc::now().timestamp_millis() as u64;

        let askMarketQuote = AskMarketQuote {
            exchange: ExchangeName::Binance,
            token: Token::asToken(self.symbol.clone()),
            price: self.askprice,
            quantity: self.askquantity,
            timestamp_ms: timestamp_millis,
        };

        AskMarketQuoteVec.push(askMarketQuote);

        AskMarketQuoteVec
    }

    pub fn getBidMarketQuotes(&self) -> Vec<BidMarketQuote> {
        let mut bidMarketQuoteVec: Vec<BidMarketQuote> = Vec::new();

        let timestamp_millis = Utc::now().timestamp_millis() as u64;

        let bidMarketQuote = BidMarketQuote {
            exchange: ExchangeName::Binance,
            token: Token::asToken(self.symbol.clone()),
            price: self.bidprice,
            quantity: self.bidquantity,
            timestamp_ms: timestamp_millis,
        };
        bidMarketQuoteVec.push(bidMarketQuote);

        bidMarketQuoteVec
    }
}
//CoinBase

//nested json requires these wrappers:

#[derive(Debug, Deserialize, Clone)]
pub struct CoinBaseMessage {
    pub events: Vec<CoinBaseEvents>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CoinBaseEvents {
    pub tickers: Vec<CoinBaseBookTicker>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CoinBaseBookTicker {
    #[serde(rename = "product_id")]
    pub symbol: String,
    #[serde(rename = "best_ask", deserialize_with = "deserialize_string_to_f64")]
    pub askprice: f64,
    #[serde(rename = "best_bid", deserialize_with = "deserialize_string_to_f64")]
    pub bidprice: f64,
    #[serde(
        rename = "best_ask_quantity",
        deserialize_with = "deserialize_string_to_f64"
    )]
    pub askquantity: f64,
    #[serde(
        rename = "best_bid_quantity",
        deserialize_with = "deserialize_string_to_f64"
    )]
    pub bidquantity: f64,
}

impl CoinBaseBookTicker {
    pub fn getBidMarketQuotes(&self) -> BidMarketQuote {
        let timestamp_millis = Utc::now().timestamp_millis() as u64;

        let bidMarketQuote = BidMarketQuote {
            exchange: ExchangeName::CoinBase,
            token: Token::asToken(self.symbol.clone()),
            price: self.bidprice,
            quantity: self.bidquantity,
            timestamp_ms: timestamp_millis,
        };

        bidMarketQuote
    }

    pub fn getAskMarketQuotes(&self) -> AskMarketQuote {
        let timestamp_millis = Utc::now().timestamp_millis() as u64;

        let askMarketQuote = AskMarketQuote {
            exchange: ExchangeName::CoinBase,
            token: Token::asToken(self.symbol.clone()),
            price: self.askprice,
            quantity: self.askquantity,
            timestamp_ms: timestamp_millis,
        };

        askMarketQuote
    }
}
