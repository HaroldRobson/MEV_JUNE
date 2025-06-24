use crate::market_data::{ExchangeName, Token};
use chrono::Utc;
use serde::{Deserialize, Deserializer, Serialize};
use serde_aux::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct AtomicMatrix {
    data: [AtomicU64; 400],
}

// this is the strange data type i am using. you'll see in market_data.rs a prime number
// mapping for each enumerated exchange and coin. by the FTA this gives me a unique index in the
// strange Atomix array below. essentially an injective (but deffo not surjective - theres a lot
// of redundant memory usage) map from a 2d object (a matrix) to a 1d vector
//
// this was necessary becasue i dont think rust has atomic matrices - only vectors.
impl AtomicMatrix {
    pub fn new() -> Self {
        const ZERO: AtomicU64 = AtomicU64::new(0);
        Self { data: [ZERO; 400] }
    }

    fn get_index(exchange: ExchangeName, token: Token) -> usize {
        let a = token as u16;
        let b = exchange as u16;
        (a as usize) * (b as usize)
    }

    pub fn update(&self, e: ExchangeName, t: Token, p: f64) {
        let p_ = p.to_bits() as u64;
        let i = Self::get_index(e, t);
        self.data[i].store(p_, Ordering::Relaxed);
    }

    pub fn get_data(&self, e: ExchangeName, t: Token) -> Option<f64> {
        let i = Self::get_index(e, t);
        let bits = self.data[i].load(Ordering::Relaxed);
        if bits == 0 {
            return None;
        } else {
            return Some(f64::from_bits(bits as u64));
        }
    }

    pub fn print_matrix(&self, name: &str) {
        println!("{} PRICES:", name);

        for exchange in ExchangeName::all() {
            for token in Token::all() {
                if let Some(price) = self.get_data(exchange, token) {
                    println!("{:?} {:?}: ${:.2}", exchange, token, price);
                }
            }
        }
        println!();
    }

    pub fn get_prices(&self, t: Token) -> [Option<(f64, ExchangeName)>; ExchangeName::COUNT] {
        let token_prime = t as usize;
        let mut prices = [None; ExchangeName::COUNT];
        let mut i = 0;
        for exchange in ExchangeName::all() {
            let exchange_prime = exchange as usize;
            let index = exchange_prime * token_prime;
            let bits = self.data[index].load(Ordering::Relaxed);
            if bits != 0 {
                prices[i] = Some((f64::from_bits(bits), exchange));
            } else {
                prices[i] = None;
            }
            i += 1;
        }
        prices
    }
    // this function obviuouly is a bit shit and is here j for testing. only finds price
    // differences on the same side (buy or ask) so isnt exactly useful for actual arbitrage
    // (ie going from buy to aask)
    pub fn find_arb_ops(&self) {
        for token in Token::all() {
            let mut min_price: f64 = 1000000000 as f64;
            let mut min_exchange: ExchangeName = ExchangeName::CoinBase;

            let mut max_price: f64 = 0 as f64;
            let mut max_exchange: ExchangeName = ExchangeName::CoinBase;
            let prices = Self::get_prices(&self, token);
            for price in prices {
                match price {
                    Some(p) => {
                        if p.0 < min_price {
                            min_price = p.0;
                            min_exchange = p.1;
                        }
                        if p.0 > max_price {
                            max_price = p.0;
                            max_exchange = p.1;
                        }
                    }
                    None => {
                        continue;
                    }
                }
            }
            let pd = (max_price - min_price) / min_price;
            if pd > 0.001 {
                println!(
                    "Found ARB! {:?} {:?} {:?} {:?} {:?} {:?}",
                    pd, min_exchange, max_exchange, token, min_price, max_price
                );
            }
        }
    }
}
