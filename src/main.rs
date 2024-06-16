use todo::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: todo <task>");
        std::process::exit(1);
    }

    let mut db = open(".todo");

    let cmd = &args[1];
    match cmd.as_str() {
        "add" => {
            if args.len() <3 {
                println!("Usage: add <task>");
                std::process::exit(1);
            }
            let contents = &args[2..].join(" ");
            let id = db.read_records().last()
                .map(|r| r.id + 1 )
                .unwrap_or(1);

            db.add_record(
                &Record {
                    id,
                    contents: contents.into(),
                }
            )
        },
        "rm"  => {
            if args.len() < 3 {
                println!("Usage: rm <id>");
                std::process::exit(1);
            }

            let id = args[2].parse::<i32>().unwrap();
            db.remove_record(id);
        },
        "ls"  => {
            let records = db.read_records();
            if records.is_empty() {
                println!("No records here!");
            } else {
                for r in records {
                    println!("{}: {}", r.id, r.contents);
                }
            }
        },
        _     => println!("unknown command"),
    }
}
