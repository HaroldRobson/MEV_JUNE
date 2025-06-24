//DISCONTINUED
//
//
//
//
//
//
//
//
/*







use crate::market_data::{ExchangeName, Token};
use chrono::Utc;
use serde::{Deserialize, Deserializer, Serialize};
use serde_aux::prelude::*;
use std::collections::HashMap;

pub struct AskSPMatrix {
    data: HashMap<(ExchangeName, Token), f64>,
}

impl AskSPMatrix {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn update(&mut self, exchange: ExchangeName, token: Token, price: f64) {
        self.data.insert((exchange, token), price);
    }

    pub fn getPrice(&self, exchange: ExchangeName, token: Token) -> Option<f64> {
        self.data.get(&(exchange, token)).copied()
    }

    // just for printin - dont worry about this too much

    pub fn print_matrix(&self) {
        if self.data.is_empty() {
            println!("ASK PRICES: No data");
            return;
        }

        // Extract all unique exchanges and tokens from the data
        let exchanges: std::collections::HashSet<ExchangeName> =
            self.data.keys().map(|(exchange, _)| *exchange).collect();
        let tokens: std::collections::HashSet<Token> =
            self.data.keys().map(|(_, token)| *token).collect();

        // Print header
        print!("{:<12}", "Exchange");
        for token in &tokens {
            print!(" {:<12}", format!("{:?}", token));
        }
        println!();

        // Print each exchange row
        for exchange in &exchanges {
            print!("{:<12}", format!("{:?}", exchange));
            for token in &tokens {
                match self.getPrice(*exchange, *token) {
                    Some(price) => print!(" ${:<11.2}", price),
                    None => print!(" {:<12}", "N/A"),
                }
            }
            println!();
        }
        println!();
    }

    pub fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

pub struct BidSPMatrix {
    data: HashMap<(ExchangeName, Token), f64>,
}
impl BidSPMatrix {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn update(&mut self, exchange: ExchangeName, token: Token, price: f64) {
        self.data.insert((exchange, token), price);
    }

    pub fn getPrice(&self, exchange: ExchangeName, token: Token) -> Option<f64> {
        self.data.get(&(exchange, token)).copied()
    }

    // just for printin - dont worry about this too much

    pub fn print_matrix(&self) {
        if self.data.is_empty() {
            println!("ASK PRICES: No data");
            return;
        }

        // Extract all unique exchanges and tokens from the data
        let exchanges: std::collections::HashSet<ExchangeName> =
            self.data.keys().map(|(exchange, _)| *exchange).collect();
        let tokens: std::collections::HashSet<Token> =
            self.data.keys().map(|(_, token)| *token).collect();

        // Print header
        print!("{:<12}", "Exchange");
        for token in &tokens {
            print!(" {:<12}", format!("{:?}", token));
        }
        println!();

        // Print each exchange row
        for exchange in &exchanges {
            print!("{:<12}", format!("{:?}", exchange));
            for token in &tokens {
                match self.getPrice(*exchange, *token) {
                    Some(price) => print!(" ${:<11.2}", price),
                    None => print!(" {:<12}", "N/A"),
                }
            }
            println!();
        }
        println!();
    }
    pub fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}
*/
