use std::io::{Write, BufReader, BufRead, Seek};
use std::fs::File;

use super::*;

pub struct Record {
    pub id: i32,
    pub contents: String,
}

pub struct Database {
    pub file: File,
}

impl Database {
    pub fn read_records(&self) -> Vec<Record> {
        let reader = BufReader::new(&self.file);
        reader.lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .map(|line| parse_record_line(&line))
            .collect()
    }

    pub fn add_record(&mut self, record: &Record) {
        let line = format!("{},{}", record.id, record.contents);
        writeln!(self.file, "{}", line).unwrap();
        println!("item added: {}", record.contents);
    }

    pub fn remove_record(&mut self, id: i32) {
        let reader = BufReader::new(&self.file);
        let mut lines = reader.lines().enumerate();
        let line = lines.find(|(_, line)| {
            let record = parse_record_line(line.as_ref().unwrap());
            record.id == id
        });

        match line {
            Some((i, _)) => {
                let contents = std::fs::read_to_string(".todo").unwrap();
                let new_contents = contents.lines()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, line)| line)
                    .collect::<Vec<_>>()
                    .join("\n");

                self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
                self.file.write_all(new_contents.as_bytes()).unwrap();
                self.file.set_len(new_contents.len() as u64).unwrap();

                println!("Item deleted: {}", id);
            }
            None => {
                println!("No such record: {}", id);
            }

        }
    }
}