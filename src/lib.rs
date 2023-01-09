use std::collections::HashMap;
use request::VType;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
pub mod request;

pub fn headers(map: &HashMap<String, String>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    for (key, value) in map.iter() {
        println!("{key}:{value}");
        headers.insert(
            HeaderName::from_bytes(key.as_bytes()).unwrap(),
            HeaderValue::from_bytes(value.as_bytes()).unwrap(),
        );
    }
    headers
}

pub fn default_headers() -> HeaderMap {
    let default_headers =
        HashMap::from([("User-Agent".to_owned(), "Http-Monitor/0.1.0".to_owned())]);
    headers(&default_headers)
}

pub fn compare_none(response: &String) -> bool {
    println!("{response}");
    true
}

pub fn contains_text(_response: &String, _text: &String) -> bool {
    //TODO
    false
}

pub fn compare_json(_response: &String, _criteria: &String, _condition: &String) -> bool {
    //TODO
    false
}

pub fn compare_regex(_response: &String, _criteria: &String, _condition: &String) -> bool {
    //TODO
    false
}

pub fn validate(response: &String, validation: &VType, criteria: &String, condition: &String) -> bool {
    let valid = match validation {
        VType::Text => contains_text(&response, &condition),
        VType::Json => compare_json(&response, &criteria,&condition),
        VType::RegEx => compare_regex(&response, &criteria,&condition),
        _ => compare_none(&response),
    };
    valid
}
