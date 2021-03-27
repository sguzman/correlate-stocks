extern crate csv;
extern crate serde;

use serde::{Serialize, Deserialize};
use std::env;
use csv::Reader;
use chrono::{Date, DateTime, TimeZone, NaiveDate};
use std::collections::HashMap;

type Hashy = HashMap<String, f64>;

#[derive(Serialize, Deserialize, Debug)]
struct Day {
    date: String,
    volume: u64,
    open: f32,
    high: f32,
    low: f32,
    close: f64,
    adjclose: f32
}

fn dot(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..a.len() {
        let a_i = a[i];
        let b_i = b[i];
        result += a_i * b_i;
    }

    return result;
}

fn get_hash(path: &str) -> Hashy {
    let csv: String = std::fs::read_to_string(path).expect("Bad file");
    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    let mut data: Hashy = HashMap::new();
    for record in reader.deserialize() {
        let record: Day = record.unwrap();
        data.insert(record.date, record.close);
    }

    return data;
}

fn arg(args: &Vec<String>, index: u32) -> &str {
    let file: &str =  args.get(index).expect("No file found");
    return file;
}

fn build(index: u32) -> Hashy {
    let args: &Vec<String> = env::args().collect();
    let path: &str = arg(args, index);
    let data: Hashy = get_hash(path);

    return data;
}

fn main() -> Result<(), csv::Error> {
    let args: &Vec<String> = env::args().collect();
    let a: Hashy = build(1);
    let b: Hashy = build(2);

    /*
    let result = dot(&a, &b) / (dot(&a, &a) * dot(&b, &b));
    println!("{}", result);

     */

    Ok(())
}