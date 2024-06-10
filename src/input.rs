// src/input.rs
use serde::Deserialize;

#[derive(Deserialize)]
pub struct InputData {
    pub name: String,
    pub r#type: String,
    pub date: String,
    pub amount: String,
    pub comment: String,
}

#[derive(Deserialize)]
pub struct AdditionalData {
    pub department: String,
    pub initiator: String,
    pub organization: String,
}

#[derive(Deserialize)]
pub struct RequestData {
    pub request: Vec<InputData>,
    pub additional: AdditionalData,
}
