use std::fs::OpenOptions;
// use anyhow::Result;
mod database;
pub use database::{Database, Record};

pub fn open(filename: &str) -> Database { 
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)
        .unwrap();

    Database {
        file,
    }
}

pub fn parse_record_line(line: &str) -> Record {
    let fields: Vec<&str> = line.split(',').collect();

    let contents = fields[1..].join(",");
    Record { 
        id: fields[0].parse::<i32>().unwrap(),
        contents,
    }
}
