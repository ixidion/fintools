use yahoo_finance_api as yahoo;
use tokio_test;
use rayon::{prelude::*, join};
use std::{thread, time, collections::HashMap};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use ::phf::{Map, phf_map};


static EXCH_MAP: Map<&'static str, &'static str> = phf_map! {
    "BTS" => "AMEX",
    "NMS" => "NASDAQ",
    "NGM" => "NASDAQ",
    "NYQ" => "NYSE"
};

fn request_symbol(isin: &str) -> (String, String) {
    let combo: String;
    let provider = yahoo::YahooConnector::new();

    let response = tokio_test::block_on(provider.search_ticker(&isin));

    
    let resp_uw = response.unwrap();

    let mut it = resp_uw.quotes.iter();
    let  yqi = it.next();

    match yqi {
        None => return (String::from(isin), String::from("error")),
        Some(entry) => {
            // println!("{:?}", entry);
            // println!("{:?}", entry.exchange);
            // println!("{}:{}", entry.exchange, entry.symbol);
            // exchange.insert_str(0, entry.exchange.as_str());
            let mut exchange = String::from(&entry.exchange);
            exchange = convert_exchange(exchange.as_str()).to_string();
            let symbol = String::from(&entry.symbol);
            combo = format!("{}:{}", exchange, symbol);
        }
    }    

    (String::from(isin), String::from(combo))
}

pub fn request_symbols(symbols: Vec<&str>) -> HashMap<String, String> {
    let map = HashMap::<String,String>::new();
    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    let (tx, rx): (Sender<(String, String)>, Receiver<(String, String)>) = mpsc::channel();
    pool.install(|| {
        symbols.par_iter().for_each_with(tx, |tx, s| {            
            // println!("{s}");
            let isin = s.split(";").nth(0).unwrap();
            tx.clone().send(request_symbol(isin)).expect("Channel sent failed.");
            }
        );
    });

    let map: HashMap<String,String> = rx.iter()
        .collect();
    map
}

fn convert_exchange(ex_in: &str) -> &str {
    let ex_out = EXCH_MAP.get(&ex_in);
    if ex_out.is_some() {
        return ex_out.unwrap();    
    } else {
        eprintln!("Error: Exchange {} was not found in the map.", ex_in);
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