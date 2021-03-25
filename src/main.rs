use serde::{Serialize, Deserialize};
use std::env;

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

fn main() -> Result<(), csv::Error> {
    let args: Vec<String> = env::args().collect();
    let a = args.get(1).expect("No first file");
    let b = args.get(2).expect("No second file");

    println!("{}, {}", a, b);

    let path_a = a;
    let path_b = b;

    let csv_a = std::fs::read_to_string(path_a).expect("Bad file");
    std::fs::read_to_string(path_b).expect("Bad file");

    let mut reader_a = csv::Reader::from_reader(csv_a.as_bytes());
    let mut reader_b = csv::Reader::from_reader(csv_a.as_bytes());

    let mut data_a = Vec::new();
    for record in reader_a.deserialize() {
        let record: Day = record?;
        data_a.push(record.close);
    }

    let mut data_b = Vec::new();
    for record in reader_b.deserialize() {
        let record: Day = record?;
        data_b.push(record.close);
    }

    let a = data_a;
    let b = data_b;

    let result = dot(&a, &b) / (dot(&a, &a) * dot(&b, &b));
    println!("{}", result);

    Ok(())
}