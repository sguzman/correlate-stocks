extern crate csv;
extern crate serde;

use serde::{Serialize, Deserialize};
use std::env;
use csv::Reader;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

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

fn get_hash(path: String) -> Hashy {
    let csv: String = std::fs::read_to_string(path).expect("Bad file");
    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    let mut data: Hashy = HashMap::new();
    for record in reader.deserialize() {
        let record: Day = record.unwrap();
        data.insert(record.date, record.close);
    }

    return data;
}

fn arg(args: Vec<String>, index: usize) -> String {
    let file: &str =  args.get(index).expect("No file found");
    return file.to_string();
}

fn build(index: usize) -> Hashy {
    let args: Vec<String> = env::args().collect();
    let path: String = arg(args, index);
    let data: Hashy = get_hash(path);

    return data;
}

fn corr(a: Vec<f64>, b: Vec<f64>) -> f64 {
    let result: f64 = dot(&a, &b) / (dot(&a, &a) * dot(&b, &b));

    return result;
}

fn keep_with_same_date(a: Hashy, b: Hashy) -> (Vec<f64>, Vec<f64>) {
    let mut keys: HashSet<String> = HashSet::new();

    for s in a.keys() {
        if b.contains_key(s) {
            keys.insert(s.clone());
        }
    }

    for s in b.keys() {
        if a.contains_key(s) {
            keys.insert(s.clone());
        }
    }

    let mut vec: Vec<String> = keys.into_iter().collect();
    vec.sort();

    let mut vec_a: Vec<f64> = Vec::new();
    let mut vec_b: Vec<f64> = Vec::new();
    for k in vec {
        let k: &str = &k;

        if a.contains_key(k) && b.contains_key(k) {
            let value: &f64 = a.get(k).unwrap();
            vec_a.push(value.clone());

            let value: &f64 = b.get(k).unwrap();
            vec_b.push(value.clone());
        }
    }

    return (vec_a, vec_b);
}

fn merge(index_a: usize, index_b: usize) -> (Vec<f64>, Vec<f64>) {
    let a: Hashy = build(index_a);
    let b: Hashy = build(index_b);

    return  keep_with_same_date(a, b);
}

fn main() -> Result<(), csv::Error> {
    let (a,b) = merge(1, 2);
    let c = corr(a, b);

    println!("{}", c);

    Ok(())
}