use std::fs::OpenOptions;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use anyhow::Result;
use dirs::home_dir;

const TODO_DB_FILENAME: &str = ".tood";

#[derive(Debug, Parser)]
#[command(version = "0.1.0", author, about, long_about = None, propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // #[command(about = "show todo info.")]
    // Info,
    #[command(about = "Add an item")]
    Add { content: String },
    #[command(about = "Remove an item")]
    Remove { id: i32 },
    #[command(about = "List all items")]
    List,
}
// use anyhow::Result;
mod database;
pub use database::{Database, Record};

pub fn open(filename: PathBuf) -> Database { 
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

fn get_db_file_path() -> std::path::PathBuf {
    home_dir()
        .map(|p| p.join(TODO_DB_FILENAME))
        .unwrap()
}

pub fn run(args: Cli) -> Result<()> {
    let mut db = open(get_db_file_path());

    match args.cmd {
        Commands::Add { content }  => {
            let id = db.read_records().last()
                .map(|r| r.id + 1 )
                .unwrap_or(1);

            db.add_record(
                &Record {
                    id,
                    contents: content.clone(),
                }
            )?;
            println!("Item added successfully: {}:{}", id, content);

            Ok(())
        },
        Commands::Remove { id }  => {
            db.remove_record(id)?;
            println!("Item removed successfully: {}", id);

            Ok(())
        },
        Commands::List  => {
            let records = db.read_records();
            if records.is_empty() {
                println!("No records here!");
            } else {
                for r in records {
                    println!("{}: {}", r.id, r.contents);
                }
            }

            Ok(())
        },
    }
}