use serde_json;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::vec::Vec;

use super::settings::get_config as cnf;
use super::utils;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct IsinNSymbol {
    isin: String,
    symbol: String,
}

pub fn read_map() -> HashMap<String, String> {
    let file_path = cnf().paths.cache_file;

    let contents = fs::read_to_string(file_path).unwrap_or_default();

    let json_contents: Vec<IsinNSymbol> = serde_json::from_str(&contents).unwrap_or_default();

    let map: HashMap<String, String> = json_contents
        .iter()
        .map(|i| (String::from(&i.isin), String::from(&i.symbol)))
        .collect();

    //println!("{:?}", map);
    map
}

pub fn write_map(map: HashMap<String, String>) {
    let file_path = cnf().paths.cache_file;
    let filename = format!("{}.json", utils::formatted_timestamp());
    let backup_file_path = utils::change_extension(file_path.clone(), &filename);

    let sorted_map = sort_map(map);

    let contents = serde_json::to_string(&sorted_map).unwrap();

    match fs::rename(&file_path, backup_file_path) {
        Ok(_) => (),
        Err(_) => eprintln!("Renaming not possible. Can be ignored normally."),
    };

    fs::write(&file_path, &contents).expect("Should have been able to write the file");
}

pub fn merge_map(to_merge: HashMap<String, String>) {
    let mut original_map: HashMap<String, String> = read_map();

    let to_merge_cleaned: HashMap<String, String> = to_merge
        .into_iter()
        .filter(|(_, v)| v.contains("error") == false)
        .collect();

    original_map.extend(to_merge_cleaned);
    write_map(original_map);
}

fn sort_map(map_in: HashMap<String, String>) -> Vec<IsinNSymbol> {
    let mut out_vec = Vec::<IsinNSymbol>::new();

    let sorted_content: BTreeMap<String, String> = map_in
        .iter()
        .map(|(k, v)| (String::from(v), String::from(k)))
        .collect();

    for (key, value) in sorted_content {
        let isin = IsinNSymbol {
            isin: String::from(value),
            symbol: String::from(key),
        };
        out_vec.push(isin);
    }

    out_vec
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn readfile() {
        let map = read_map();
        let my_str = map.get("US87968A1043");
        assert_eq!(my_str.unwrap(), "AMEX:TELL");
        write_map(map);
    }

    #[test]
    fn writefile() {
        let mut in_map: HashMap<String, String> = HashMap::new();
        in_map.insert(String::from("1"), String::from("a"));
        in_map.insert(String::from("2"), String::from("b"));
        write_map(in_map);
    }

    #[test]
    fn print_json() {
        let mut myvec = Vec::<IsinNSymbol>::new();
        let i1 = IsinNSymbol {
            isin: String::from("i"),
            symbol: String::from("s"),
        };
        let i2 = IsinNSymbol {
            isin: String::from("i1"),
            symbol: String::from("s1"),
        };
        myvec.push(i1);
        myvec.push(i2);
        let os = serde_json::to_string(&myvec).unwrap();

        println!("{}", os);
    }
}
