extern crate chrono;
extern crate csv;
extern crate serde;

use serde::{Serialize, Deserialize};
use std::env;
use csv::Reader;
use chrono::{Date, DateTime, TimeZone, NaiveDate};

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

#[derive(Debug)]
struct Data {
    date: NaiveDate,
    close: f64
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

fn get_recs(path: &String) -> Vec<Data> {
    let csv = std::fs::read_to_string(path).expect("Bad file");
    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    let mut data = Vec::new();
    for record in reader.deserialize() {
        let record: Day = record.unwrap();

        let date = NaiveDate::parse_from_str(&*record.date, "%Y-%m-%d").unwrap();
        let close = record.close;

        let value = Data {
            date,
            close
        };
        data.push(value);
    }

    return data
}

fn main() -> Result<(), csv::Error> {
    let args: Vec<String> = env::args().collect();
    let a = args.get(1).expect("No first file");
    let b = args.get(2).expect("No second file");

    println!("{}, {}", a, b);
    let a = get_recs(a);
    let b = get_recs(b);
    /*



    let result = dot(&a, &b) / (dot(&a, &a) * dot(&b, &b));
    println!("{}", result);

     */

    Ok(())
}