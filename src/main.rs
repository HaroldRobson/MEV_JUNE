use futures::{SinkExt, StreamExt};
use http::Uri;
use serde_json;
use serde_json::json;
use tokio::task::JoinHandle;
use tokio::*;
use tokio_websockets::{ClientBuilder, Error, Message};
mod market_data;
use crate::market_data::BinanceBookTicker;
use market_data::{
    AskMarketQuote, BidMarketQuote, CoinBaseBookTicker, CoinBaseMessage, ExchangeName,
};
mod ATOMIC_SP_MATRIX;
use std::sync::Arc;
use tokio::sync::mpsc;
use ATOMIC_SP_MATRIX::AtomicMatrix;
//mod SPmatrix;
//use SPmatrix::{AskSPMatrix, BidSPMatrix};
#[tokio::main]
async fn main() {
    // internal MarketQuote stream:
    //let (ask_sender, mut ask_receiver) = mpsc::unbounded_channel::<AskMarketQuote>();
    //let (bid_sender, mut bid_receiver) = mpsc::unbounded_channel::<BidMarketQuote>();

    let bid_atomic_matrix = Arc::new(AtomicMatrix::new());
    let ask_atomic_matrix = Arc::new(AtomicMatrix::new());

    //binance streams:
    let streams = vec![("BTC/USDT", "wss://stream.binance.com:9443/ws/stream")];
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for stream in streams {
        //let ask_sender_clone = ask_sender.clone();
        //let bid_sender_clone = bid_sender.clone();
        let ask_atomic_matrix_clone = ask_atomic_matrix.clone();
        let bid_atomic_matrix_clone = bid_atomic_matrix.clone();
        let handle = tokio::spawn(async move {
            let uri = Uri::from_static(stream.1);
            let (mut ws_stream, response) = match ClientBuilder::from_uri(uri).connect().await {
                Ok(conn) => conn,
                Err(e) => {
                    eprintln!("{:?}", e);
                    return;
                }
            };
            println!("connected to binance");
            let subscribe_msg = json!({
                "method": "SUBSCRIBE",
                "params": [
                    "btcusdt@bookTicker",
                    "ethusdt@bookTicker",
                    "adausdt@bookTicker",
                    "linkusdt@bookTicker"
                ],
                "id": 1
            });
            ws_stream
                .send(Message::text(subscribe_msg.to_string()))
                .await;
            while let Some(msg) = ws_stream.next().await {
                match msg {
                    Ok(message) => {
                        if let Some(text) = message.as_text() {
                            if text.contains("\"e\":\"trade\"") {
                                continue;
                            }

                            match serde_json::from_str::<BinanceBookTicker>(text) {
                                Ok(book_ticker) => {
                                    let ask_market_quotes = book_ticker.getAskMarketQuotes();
                                    for ask_market_quote in ask_market_quotes {
                                        ask_atomic_matrix_clone.update(
                                            ask_market_quote.exchange,
                                            ask_market_quote.token,
                                            ask_market_quote.price,
                                        );
                                        //ask_sender_clone.send(ask_market_quote).unwrap();
                                    }
                                    let bid_market_quotes = book_ticker.getBidMarketQuotes();
                                    for bid_market_quote in bid_market_quotes {
                                        bid_atomic_matrix_clone.update(
                                            bid_market_quote.exchange,
                                            bid_market_quote.token,
                                            bid_market_quote.price,
                                        );
                                        // bid_sender_clone.send(bid_market_quote).unwrap();
                                    }
                                }

                                Err(e) => {
                                    eprintln!("failed to parse json {:?}", e);
                                }
                            }
                        } else if message.is_close() {
                            println!("closing");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                        break;
                    }
                }
            }
        });

        handles.push(handle);
    }

    //coinbase streams:
    let CBstreams = vec![("", "wss://advanced-trade-ws.coinbase.com")];
    for stream in CBstreams {
        //let ask_sender_clone = ask_sender.clone();
        //let bid_sender_clone = bid_sender.clone();
        let ask_atomic_matrix_clone = ask_atomic_matrix.clone();
        let bid_atomic_matrix_clone = bid_atomic_matrix.clone();
        let mut handle = tokio::spawn(async move {
            let uri = Uri::from_static(stream.1);
            let (mut ws_stream, response) = match ClientBuilder::from_uri(uri).connect().await {
                Ok(conn) => conn,
                Err(e) => {
                    eprintln!("{:?}", e);
                    return;
                }
            };
            println!("connected to coinbase!");
            let subscribe_msg = json!({
                "type": "subscribe",
                "product_ids": ["BTC-USDT", "ETH-USDT","ADA-USDT", "LINK-USDT" ],
                "channel": "ticker"
            });
            ws_stream
                .send(Message::text(subscribe_msg.to_string()))
                .await;
            while let Some(msg) = ws_stream.next().await {
                match msg {
                    Ok(message) => {
                        if message.is_text() {
                            if let Some(text) = message.as_text() {
                                if text.contains("\"e\":\"trade\"") {
                                    continue;
                                }

                                match serde_json::from_str::<CoinBaseMessage>(text) {
                                    Ok(coinbase_msg) => {
                                        for event in coinbase_msg.events {
                                            for ticker in event.tickers {
                                                let market_quote = ticker.getAskMarketQuotes();
                                                ask_atomic_matrix_clone.update(
                                                    market_quote.exchange,
                                                    market_quote.token,
                                                    market_quote.price,
                                                );
                                                //ask_sender_clone.send(market_quote).unwrap();
                                                let market_quote = ticker.getBidMarketQuotes();
                                                bid_atomic_matrix_clone.update(
                                                    market_quote.exchange,
                                                    market_quote.token,
                                                    market_quote.price,
                                                );
                                                //bid_sender_clone.send(market_quote).unwrap();
                                            }
                                        }
                                    }

                                    Err(e) => {
                                        eprintln!("failed to parse json from coinbase {:?}", e);
                                    }
                                }
                            }
                        } else if message.is_close() {
                            println!("closing");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("{:?}", e);
                        break;
                    }
                }
            }
        });

        handles.push(handle);
    }

    let bam_clone = bid_atomic_matrix.clone(); // for use in the arb_opp_finder thread.
    let print_receiver_handle = tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(20));
        loop {
            interval.tick().await;
            bid_atomic_matrix.print_matrix("BID");
            ask_atomic_matrix.print_matrix("ASK");
        }
    });
    let arb_opp_finder = tokio::spawn(async move {
        loop {
            bam_clone.find_arb_ops();
        }
    });

    handles.push(print_receiver_handle);

    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("Task failed: {:?}", e);
        }
    }
}
