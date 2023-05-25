use crate::finlibs::settings::get_config as cnf;

use super::cache;
use super::utils;
use ::phf::{phf_map, Map};
use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, RwLock};
use std::{collections::HashMap, collections::HashSet};
use tokio_test;
use yahoo_finance_api as yahoo;

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
    let yqi = it.next();

    match yqi {
        None => return (String::from(isin), isin.to_string() + "error"),
        Some(entry) => {
            let mut exchange = String::from(&entry.exchange);
            exchange = convert_exchange(exchange.as_str()).to_string();
            let mut symbol = String::from(&entry.symbol);
            convert_symbol(&mut symbol);
            combo = format!("{}:{}", exchange, symbol);
        }
    }

    (String::from(isin), String::from(combo))
}

fn request_symbol_cached(
    isin: &str,
    isin_cache: &Arc<RwLock<HashMap<String, String>>>,
) -> (String, String) {
    if let Some(symbol) = isin_cache.read().unwrap().get(isin) {
        //println!("Cache-Hit: {} {}", &isin, &symbol);
        return (isin.to_owned(), symbol.to_owned());
    } else {
        return request_symbol(isin);
    }
}

pub fn request_symbols(symbols: Vec<&str>) -> HashMap<String, String> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
    let (tx, rx): (Sender<(String, String)>, Receiver<(String, String)>) = mpsc::channel();
    pool.install(|| {
        let cached_isins = Arc::new(RwLock::new(cache::read_map()));
        let local_isin = Arc::clone(&cached_isins);
        symbols.par_iter().for_each_with(tx, move |tx, s| {
            // println!("{s}");
            let isin = s.split(";").nth(0).unwrap();
            if isin.len() == 12 {
                tx.clone()
                    .send(request_symbol_cached(isin, &local_isin))
                    .expect("Channel sent failed.");
            }
        });
    });

    let map: HashMap<String, String> = rx.iter().collect();
    cache::merge_map(map.clone());
    //println!("{:?}", map);
    map
}

fn convert_exchange(ex_in: &str) -> &str {
    let ex_out = EXCH_MAP.get(&ex_in);
    if ex_out.is_some() {
        return ex_out.unwrap();
    } else {
        eprintln!(
            "Error: Exchange {} was not found in the map in finance.rs.",
            ex_in
        );
        return "error";
    }
}

fn convert_symbol(symbol: &mut String) {
    *symbol = symbol.replace("-", ".");
}

pub fn compare(fn_vec: &Vec<PathBuf>) {
    assert_eq!(fn_vec.len(), 2);
    let filename1_buff = fn_vec.get(0).unwrap();
    let filename2_buff = fn_vec.get(1).unwrap();

    // let filename1 = extract_filenames(filename1_buff).expect("Nothing found here");
    // let filename2 = extract_filenames(filename2_buff).expect("Nothing found here");

    let fn_str_vec: Vec<&str> = fn_vec
        .iter()
        .map(|pb| pb.file_name().unwrap().to_str().unwrap())
        .collect::<Vec<&str>>();
    //let  f1_is_higher = finance::compare(&fn_str_vec);

    if find_highest_filename(&fn_str_vec) > -1 {
        compare_files(filename1_buff, filename2_buff);
    } else {
        compare_files(filename2_buff, filename1_buff);
    }
}

fn compare_files(filename1_buff: &PathBuf, filename2_buff: &PathBuf) {
    let file_content1 = fs::read_to_string(filename1_buff).unwrap();
    let file_content2 = fs::read_to_string(filename2_buff).unwrap();
    let fc1_col: HashSet<&str> = file_content1.lines().collect();
    let fc2_col: HashSet<&str> = file_content2.lines().collect();
    let new_entries: String = fc1_col
        .difference(&fc2_col)
        .fold(String::from("###NEW\n"), |acc, x| acc + x + "\n");
    let gone_entries: String = fc2_col
        .difference(&fc1_col)
        .fold(String::from("###GONE\n"), |acc, x| acc + x + "\n");

    let contents = new_entries + &gone_entries;

    let filename = cnf().vars.prefix_diff
        + utils::formatted_timestamp().as_str()
        + &cnf().vars.suffix;
    let full_path = cnf().paths.output_path_diff.join(filename);

    fs::write(full_path, &contents).expect("Should have been able to write the file");

    println!("New: {:?}", &contents);
}

fn find_highest_filename(fn_vec: &Vec<&str>) -> i8 {
    assert_eq!(fn_vec.len(), 2);
    let filename1_buff = fn_vec.get(0).unwrap();
    let filename2_buff = fn_vec.get(1).unwrap();

    let filename1 = extract_filenames(filename1_buff).expect("Nothing found here. Filename 1.");
    let filename2 = extract_filenames(filename2_buff).expect("Nothing found here. Filename 2.");
    if filename1 > filename2 {
        1
    } else if filename1 < filename2 {
        -1
    } else {
        0
    }
}

fn extract_filenames(text: &str) -> Option<&str> {
    lazy_static! {
        static ref TIMESTAMP_REGEX: Regex = Regex::new(r"stocks(\d+)").unwrap();
    }
    let caps = TIMESTAMP_REGEX.captures(text);
    if caps.is_some() {
        return Some(caps.unwrap().get(1).unwrap().as_str());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::Path;
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
        //let res = convert_exchange("BLA");
        assert_eq!(convert_exchange("NMS"), "NASDAQ");
    }

    #[test]
    fn test_regexp() {
        println!("{}", extract_filenames("stocks234.txt").unwrap());
        if "20221031102807".parse::<u64>().unwrap() > "20221030133844".parse::<u64>().unwrap() {
            println!(
                "Yes! {} {}",
                "20221031102807".parse::<u64>().unwrap(),
                "20221030133844".parse::<u64>().unwrap()
            )
        }
    }

    #[test]
    fn test_compare() {
        let in_vec: Vec<&str> = vec!["stocks20221031102807.txt", "stocks20220911095127.txt"];
        find_highest_filename(&in_vec);
        let in_vec: Vec<&str> = vec!["stocks20220911095127.txt", "stocks20221031102807.txt"];
        find_highest_filename(&in_vec);
        println!("{:?}", Path::new("a").join("b"));
    }

    #[test]
    fn test_compare_files() {
        let p_high = Path::new("C:\\temp\\stocks20221106092936.txt").to_path_buf();
        let p_low = Path::new("C:\\temp\\stocks20221106092935.txt").to_path_buf();
        compare_files(&p_high, &p_low);
    }
}
