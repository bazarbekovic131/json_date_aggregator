mod input;
mod output;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{Datelike, NaiveDate};
use env_logger::Env;
use input::RequestData;
use log::info;
use output::OutputData;
// use serde::Serialize;
use std::collections::HashMap;

fn get_days_in_month(year: i32, month: u8) -> u8 {
    let date = NaiveDate::from_ymd_opt(year, month.into(), 1).unwrap();
    date.with_month((month + 1).into())
        .unwrap_or_else(|| date.with_year(year + 1).unwrap())
        .signed_duration_since(date)
        .num_days() as u8
}

fn parse_date(date_str: &str) -> (i32, u8, u8) {
    let date = NaiveDate::parse_from_str(date_str, "%d/%m/%Y").unwrap();
    (date.year(), date.month() as u8, date.day() as u8)
}

async fn transform_data(data: web::Json<RequestData>) -> impl Responder {
    info!("Получен запрос для трансформации данных");
    let mut output_map: HashMap<(String, String), Vec<f32>> = HashMap::new();
    let mut max_days = 0;

    for entry in data.request.iter() {
        let key = (entry.name.clone(), entry.r#type.clone());
        let (year, month, day) = parse_date(&entry.date);
        // let comment = entry.comment.clone();
        info!(
            "Обрабатываю вводные: Наименование={}, Ед. изм={}, Дата={}, Кол-во={}",
            entry.name, entry.r#type, entry.date, entry.amount
        );

        let days = output_map.entry(key).or_insert(vec![0.0; 31]);

        if max_days == 0 {
            max_days = get_days_in_month(year, month);
        }

        if day as usize <= max_days as usize {
            days[day as usize - 1] += entry.amount.parse::<f32>().unwrap();
        }
    }

    let output_data: Vec<OutputData> = output_map
        .into_iter()
        .map(|((name, r#type), days)| OutputData {
            name,
            r#type,
            total: days.iter().sum(),
            days,
        })
        .collect();
    info!("Обработка запроса завершена успешно");

    HttpResponse::Ok().json(serde_json::json!({
        "length": output_data.len(),
        "data": output_data,
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let host = "127.0.0.1:8080";

    info!("Сервер запускается на http://{}", host);
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .route("/transform", web::post().to(transform_data))
    })
    .bind(host)?
    .run()
    .await
}
