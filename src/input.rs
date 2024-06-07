// src/input.rs
use serde::Deserialize;

#[derive(Deserialize)]
pub struct InputData {
    pub name: String,
    pub r#type: String,
    pub date: String,
    pub amount: f64,
    pub comment: String,
}

#[derive(Deserialize)]
struct AdditionalData {
    department: String,
    initiator: String,
    organization: String,
}

#[derive(Deserialize)]
pub struct RequestData {
    pub request: Vec<InputData>,
    pub additional: AdditionalData,
}
