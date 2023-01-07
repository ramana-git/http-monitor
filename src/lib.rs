use std::collections::HashMap;
use request::{Validation, VType};
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

pub fn compare_text(_response: &String, _text: &String) -> bool {
    //TODO
    false
}

pub fn compare_json(_response: &String, _condition: &String, _text: &String) -> bool {
    //TODO
    false
}

pub fn compare_regex(_response: &String, _condition: &String, _text: &String) -> bool {
    //TODO
    false
}

pub fn validate(response: &String, validation: &Validation) -> bool {
    let valid = match validation.vtype {
        VType::Text => compare_text(response, &validation.value),
        VType::Json => compare_json(response, &validation.condition, &validation.value),
        VType::RegEx => compare_regex(response,&validation.condition, &validation.value),
        _ => compare_none(response),
    };
    valid
}
