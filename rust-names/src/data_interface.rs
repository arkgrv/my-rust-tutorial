use reqwest;
use json;

pub fn get_data(url: &str) -> String {
    // Use reqwest blocking client to get data from url and 
    // return it as a string.
    use std::io::Read;
    let client = reqwest::blocking::Client::new();
    let mut response = client.get(url).send().unwrap();
    let mut data = String::new();
    response.read_to_string(&mut data).unwrap();
    data
}

pub fn data_to_json(data: &str) -> Result<json::JsonValue, json::JsonError> {
    let json_data = json::parse(data)?;
    Ok(json_data)
}