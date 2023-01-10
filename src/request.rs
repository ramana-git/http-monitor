use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub enum VType {
    None,
    Text,
    Json,
    RegEx,
}

#[derive(Deserialize, Debug)]
pub struct HealthRequest {
    pub uuid: Uuid,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub interval: i32, //seconds default 120 min
    pub timeout: i32,  //seconds default 5 seconds
    pub validation: VType,
    pub criteria: String,
    pub condition: String,
}
impl Default for HealthRequest {
    fn default() -> Self {
        HealthRequest {
            uuid: Uuid::nil(),
            url: String::new(),
            headers: HashMap::new(),
            interval: 120,
            timeout: 5,
            validation: VType::None,
            criteria: String::new(),
            condition: String::new(),
        }
    }
}
impl HealthRequest {}
