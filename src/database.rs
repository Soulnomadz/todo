use std::io::{Write, BufReader, BufRead, Seek};
use std::fs::File;
use anyhow::{Context, Result, anyhow};

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

    pub fn add_record(&mut self, record: &Record) -> Result<()> {
        let line = format!("{},{}", record.id, record.contents);
        writeln!(self.file, "{}", line)
            .with_context(|| "failed to add record")
    }

    pub fn remove_record(&mut self, id: i32) -> Result<()> {
        let reader = BufReader::new(&self.file);
        let mut lines = reader.lines().enumerate();
        let line = lines.find(|(_, line)| {
            let record = parse_record_line(line.as_ref().unwrap());
            record.id == id
        });

        match line {
            Some((i, _)) => {
                let contents = std::fs::read_to_string(get_db_file_path()).unwrap();
                let new_contents = contents.lines()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, line)| line)
                    .collect::<Vec<_>>()
                    .join("\n");

                self.file.seek(std::io::SeekFrom::Start(0))?;
                self.file.write_all(new_contents.as_bytes())?;
                self.file.set_len(new_contents.len() as u64)?;

                Ok(())
            }
            None => {
                Err(anyhow!("No such record: {}", id))
            }

        }
    }
}