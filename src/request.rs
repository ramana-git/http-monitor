use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub enum VType {
    None,
    Text,
    Json,
    RegEx,
}

#[derive(Deserialize, Debug)]
pub struct Validation{
    pub vtype: VType,
    pub condition: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct HealthRequest {
    pub url: String,
    pub headers: HashMap<String, String>,
    pub interval: u64, //seconds default 120 min
    pub timeout: u64,  //seconds default 5 seconds
    pub validation: Validation,
}
impl Default for HealthRequest {
    fn default() -> Self {
        HealthRequest {
            url: String::new(),
            headers: HashMap::new(),
            interval: 120,
            timeout: 5,
            validation: Validation{
                vtype: VType::None,
                condition: String::new(),
                value: String::new()
            }
        }
    }
}
impl HealthRequest {}
