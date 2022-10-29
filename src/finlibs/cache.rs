use std::collections::HashMap;
use std::collections::BTreeMap;
use std::vec::Vec;
use std::fs;
use serde_json;

use super::utils;
use super::envs;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct IsinNSymbol {
    isin: String, 
    symbol: String
}

pub fn read_map() -> HashMap<String, String> {
    let file_path = crate::envs::get_config().cache_file;

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let json_contents: Vec<IsinNSymbol> = serde_json::from_str(&contents)
        .expect("Something went wrong during JSON-Parsing.");

    let map: HashMap<String, String> = json_contents
        .iter()
        .map(|i| (String::from(&i.isin), String::from(&i.symbol)))
        .collect();
    map
}

pub fn write_map(map: HashMap<String,String>) {

    let file_path = envs::get_config().cache_file;
    let filename = format!("{}.json", utils::formatted_timestamp());
    let backup_file_path = utils::change_extension(file_path.clone(), &filename);
   
    let sorted_map = sort_map(map);
    
    let contents = serde_json::to_string(&sorted_map).unwrap();

    fs::rename(&file_path, backup_file_path)
        .expect("Renaming failed.");

    fs::write(&file_path, contents)
        .expect("Should have been able to write the file");

}

fn sort_map(map_in: HashMap<String, String>) -> Vec<IsinNSymbol> {
    let mut out_vec = Vec::<IsinNSymbol>::new();

    let sorted_content: BTreeMap<String, String> = map_in
        .iter()
        .map(|(k,v)| (String::from(v), String::from(k)))
        .collect();


    for (key, value) in sorted_content {
        let isin = IsinNSymbol {
            isin: String::from(value),
            symbol: String::from(key)
        };
        out_vec.push(isin);
    }    

    out_vec    
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn readfile() {
        let map = read_map();
        let my_str = map.get("US87968A1043");
        assert_eq!(my_str.unwrap(), "AMEX:TELL");
        write_map(map);
    }    

    #[test]
    fn writefile() {
        let mut in_map: HashMap<String, String> =  HashMap::new();
        in_map.insert(String::from("1"), String::from("a"));
        in_map.insert(String::from("2"), String::from("b"));
        write_map(in_map);
    }

    #[test]
    fn print_json() {
        let mut myvec = Vec::<IsinNSymbol>::new();
        let i1 = IsinNSymbol {
            isin: String::from("i"),
            symbol: String::from("s")

        };
        let i2 = IsinNSymbol {
            isin: String::from("i1"),
            symbol: String::from("s1")


        };
        myvec.push(i1);
        myvec.push(i2);
        let os = serde_json::to_string(&myvec).unwrap();

        println!("{}", os);
    }
}
