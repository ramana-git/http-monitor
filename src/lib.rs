use request::VType;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;

pub mod request;
pub mod trackers;

pub fn headers(map: &HashMap<String, String>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    for (key, value) in map.iter() {
        headers.insert(
            HeaderName::from_bytes(key.as_bytes()).unwrap(),
            HeaderValue::from_bytes(value.as_bytes()).unwrap(),
        );
    }
    headers
}

pub fn default_headers() -> HeaderMap {
    let user_agent = env!("CARGO_PKG_NAME").to_owned() + "/" + env!("CARGO_PKG_VERSION");
    let default_headers = HashMap::from([("User-Agent".to_owned(), user_agent)]);
    headers(&default_headers)
}

pub fn compare_none(_response: &String) -> bool {
    //println!("{response}");
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

pub fn validate(
    response: &String,
    validation: &VType,
    criteria: &String,
    condition: &String,
) -> bool {
    match validation {
        VType::Text => contains_text(&response, &condition),
        VType::Json => compare_json(&response, &criteria, &condition),
        VType::RegEx => compare_regex(&response, &criteria, &condition),
        _ => compare_none(&response),
    }
}
