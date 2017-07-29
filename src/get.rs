use std::io::Read;
use serde_json::Value;
use serde_json;
use reqwest;

pub fn get_data(url: &str) -> Result<Value, ()> {
    let mut r = reqwest::get(url).unwrap();
    let mut d = String::new();
    r.read_to_string(&mut d).unwrap();
    println!("{} / {}", url, d);

    let j: Value = serde_json::from_str(&d).unwrap();
    match j["status"].to_string().as_str() {
        r#""ok""# => Ok(j.clone()),
        _ => Err(()),
    }
}