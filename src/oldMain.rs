mod input;
mod output;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::{Datelike, NaiveDate};
use env_logger::Env;
use input::RequestData;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
struct OutputData28 {
    name: String,
    r#type: String,
    total: f32,
    day1: f32,
    day2: f32,
    day3: f32,
    day4: f32,
    day5: f32,
    day6: f32,
    day7: f32,
    day8: f32,
    day9: f32,
    day10: f32,
    day11: f32,
    day12: f32,
    day13: f32,
    day14: f32,
    day15: f32,
    day16: f32,
    day17: f32,
    day18: f32,
    day19: f32,
    day20: f32,
    day21: f32,
    day22: f32,
    day23: f32,
    day24: f32,
    day25: f32,
    day26: f32,
    day27: f32,
    day28: f32,
    // comment: String,
}

#[derive(Serialize)]
struct OutputData29 {
    name: String,
    r#type: String,
    total: f32,
    day1: f32,
    day2: f32,
    day3: f32,
    day4: f32,
    day5: f32,
    day6: f32,
    day7: f32,
    day8: f32,
    day9: f32,
    day10: f32,
    day11: f32,
    day12: f32,
    day13: f32,
    day14: f32,
    day15: f32,
    day16: f32,
    day17: f32,
    day18: f32,
    day19: f32,
    day20: f32,
    day21: f32,
    day22: f32,
    day23: f32,
    day24: f32,
    day25: f32,
    day26: f32,
    day27: f32,
    day28: f32,
    day29: f32,
    // comment: String,
}

#[derive(Serialize)]
struct OutputData30 {
    name: String,
    r#type: String,
    total: f32,
    day1: f32,
    day2: f32,
    day3: f32,
    day4: f32,
    day5: f32,
    day6: f32,
    day7: f32,
    day8: f32,
    day9: f32,
    day10: f32,
    day11: f32,
    day12: f32,
    day13: f32,
    day14: f32,
    day15: f32,
    day16: f32,
    day17: f32,
    day18: f32,
    day19: f32,
    day20: f32,
    day21: f32,
    day22: f32,
    day23: f32,
    day24: f32,
    day25: f32,
    day26: f32,
    day27: f32,
    day28: f32,
    day29: f32,
    day30: f32,
    // comment: String,
}

#[derive(Serialize)]
struct OutputData31 {
    name: String,
    r#type: String,
    total: f32,
    day1: f32,
    day2: f32,
    day3: f32,
    day4: f32,
    day5: f32,
    day6: f32,
    day7: f32,
    day8: f32,
    day9: f32,
    day10: f32,
    day11: f32,
    day12: f32,
    day13: f32,
    day14: f32,
    day15: f32,
    day16: f32,
    day17: f32,
    day18: f32,
    day19: f32,
    day20: f32,
    day21: f32,
    day22: f32,
    day23: f32,
    day24: f32,
    day25: f32,
    day26: f32,
    day27: f32,
    day28: f32,
    day29: f32,
    day30: f32,
    day31: f32,
    // comment: String,
}

#[derive(Serialize)]
#[serde(untagged)]
enum OutputData {
    Data28(OutputData28),
    Data29(OutputData29),
    Data30(OutputData30),
    Data31(OutputData31),
}

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
            days[day as usize - 1] += entry.amount as f32;
        }
    }

    let output_data: Vec<OutputData> = output_map
        .into_iter()
        .map(|((name, r#type), days)| match max_days {
            28 => OutputData::Data28(OutputData28 {
                name,
                r#type,
                total: days.iter().sum(),
                day1: days[0],
                day2: days[1],
                day3: days[2],
                day4: days[3],
                day5: days[4],
                day6: days[5],
                day7: days[6],
                day8: days[7],
                day9: days[8],
                day10: days[9],
                day11: days[10],
                day12: days[11],
                day13: days[12],
                day14: days[13],
                day15: days[14],
                day16: days[15],
                day17: days[16],
                day18: days[17],
                day19: days[18],
                day20: days[19],
                day21: days[20],
                day22: days[21],
                day23: days[22],
                day24: days[23],
                day25: days[24],
                day26: days[25],
                day27: days[26],
                day28: days[27],
                // comment,
            }),
            29 => OutputData::Data29(OutputData29 {
                name,
                r#type,
                total: days.iter().sum(),
                day1: days[0],
                day2: days[1],
                day3: days[2],
                day4: days[3],
                day5: days[4],
                day6: days[5],
                day7: days[6],
                day8: days[7],
                day9: days[8],
                day10: days[9],
                day11: days[10],
                day12: days[11],
                day13: days[12],
                day14: days[13],
                day15: days[14],
                day16: days[15],
                day17: days[16],
                day18: days[17],
                day19: days[18],
                day20: days[19],
                day21: days[20],
                day22: days[21],
                day23: days[22],
                day24: days[23],
                day25: days[24],
                day26: days[25],
                day27: days[26],
                day28: days[27],
                day29: days[28],
                // comment,
            }),
            30 => OutputData::Data30(OutputData30 {
                name,
                r#type,
                total: days.iter().sum(),
                day1: days[0],
                day2: days[1],
                day3: days[2],
                day4: days[3],
                day5: days[4],
                day6: days[5],
                day7: days[6],
                day8: days[7],
                day9: days[8],
                day10: days[9],
                day11: days[10],
                day12: days[11],
                day13: days[12],
                day14: days[13],
                day15: days[14],
                day16: days[15],
                day17: days[16],
                day18: days[17],
                day19: days[18],
                day20: days[19],
                day21: days[20],
                day22: days[21],
                day23: days[22],
                day24: days[23],
                day25: days[24],
                day26: days[25],
                day27: days[26],
                day28: days[27],
                day29: days[28],
                day30: days[29],
                // comment,
            }),
            31 => OutputData::Data31(OutputData31 {
                name,
                r#type,
                total: days.iter().sum(),
                day1: days[0],
                day2: days[1],
                day3: days[2],
                day4: days[3],
                day5: days[4],
                day6: days[5],
                day7: days[6],
                day8: days[7],
                day9: days[8],
                day10: days[9],
                day11: days[10],
                day12: days[11],
                day13: days[12],
                day14: days[13],
                day15: days[14],
                day16: days[15],
                day17: days[16],
                day18: days[17],
                day19: days[18],
                day20: days[19],
                day21: days[20],
                day22: days[21],
                day23: days[22],
                day24: days[23],
                day25: days[24],
                day26: days[25],
                day27: days[26],
                day28: days[27],
                day29: days[28],
                day30: days[29],
                day31: days[30],
                // comment,
            }),
            _ => unreachable!(),
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
