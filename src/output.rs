// src/output.rs
// Created by Bazarbekov A.
//
// 6. June, 2024
//
// OutputData is the struct of response JSON sent from this application
// to the requesting server (Doc-V) upon retrieval of the request.
//
// types defined in the struct:
// name - name of the item
// type - edinica izmerenija - stucka, metr, and so on
// total - total amount by an item
// days - Vector (array in a common sense) containing amount per day in the given month

use serde::Serialize;

#[derive(Serialize)]
pub struct OutputData {
    pub name: String,
    pub r#type: String,
    pub total: f32,
    pub days: Vec<f32>,
    // pub comment: String,
}

// #[derive(Serialize)]
// struct OutputData {
//     name: String,
//     r#type: String,
//     total: f32,
//     days: Vec<f32>,
//     // pub comment: String,
// }
