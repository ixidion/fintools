use yahoo_finance_api as yahoo;
use tokio_test;
use rayon::{prelude::*};
use std::{collections::HashMap};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, RwLock, Arc};
use ::phf::{Map, phf_map};

use super::cache;


static EXCH_MAP: Map<&'static str, &'static str> = phf_map! {
    "BTS" => "AMEX",
    "NCM" => "NASDAQ",
    "NMS" => "NASDAQ",
    "NGM" => "NASDAQ",
    "NYQ" => "NYSE",
};

fn request_symbol(isin: &str) -> (String, String) {
    let combo: String;
    let provider = yahoo::YahooConnector::new();

    let response = tokio_test::block_on(provider.search_ticker(&isin));

    
    let resp_uw = response.unwrap();

    let mut it = resp_uw.quotes.iter();
    let  yqi = it.next();

    match yqi {
        None => return (String::from(isin), isin.to_string() + "error"),
        Some(entry) => {
            let mut exchange = String::from(&entry.exchange);
            exchange = convert_exchange(exchange.as_str()).to_string();
            let symbol = String::from(&entry.symbol);
            combo = format!("{}:{}", exchange, symbol);
        }
    }    

    (String::from(isin), String::from(combo))
}

fn request_symbol_cached(isin: &str, isin_cache: &Arc<RwLock<HashMap<String, String>>>) -> (String, String) {    
    if let Some(symbol) = isin_cache.read().unwrap().get(isin) {
        //println!("Cache-Hit: {} {}", &isin, &symbol);
        return (isin.to_owned(), symbol.to_owned());
    } else {
        return request_symbol(isin);
    }
}


pub fn request_symbols(symbols: Vec<&str>) -> HashMap<String, String> {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    let (tx, rx): (Sender<(String, String)>, Receiver<(String, String)>) = mpsc::channel();
    pool.install(|| {
        let cached_isins = Arc::new(RwLock::new(cache::read_map()));
        let local_isin = Arc::clone(&cached_isins);
        symbols.par_iter().for_each_with(tx, move |tx, s| {            
            // println!("{s}");
            let isin = s.split(";").nth(0).unwrap();
            if isin.len() == 12 {
                tx.clone().send(request_symbol_cached(isin, &local_isin)).expect("Channel sent failed.");
            }
        }
        );
    });

    let map: HashMap<String,String> = rx.iter()
        .collect();
    cache::merge_map(map.clone());
    //println!("{:?}", map);
    map
}

fn convert_exchange(ex_in: &str) -> &str {
    let ex_out = EXCH_MAP.get(&ex_in);
    if ex_out.is_some() {
        return ex_out.unwrap();    
    } else {
        eprintln!("Error: Exchange {} was not found in the map in finance.rs.", ex_in);
        return "error";
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    #[test]
    fn symbol() {
        env::set_var("RUST_BACKTRACE", "full");
        // crate::finance::request_symbol(String::from("US0378331005"));
    }    

    #[test]
    fn symbols() {
        let in_vec: Vec<&str> = vec!["US0378331005", "US26856L1035", "US6304021057"];
        request_symbols(in_vec);
    }

    #[test]
    fn test_convert_exchange() {
        let res = convert_exchange("BLA");
        assert_eq!(convert_exchange("NMS"), "NASDAQ");
    }

}