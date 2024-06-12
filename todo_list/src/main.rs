use std::{any::Any, env::args, process::exit};

use rusqlite::Connection;

macro_rules! err {
    ($($args: tt)*) => {
        eprintln!($($args)*);
        exit(1);
    }
}

struct Task {
    id: i32,
    task: String,
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        err!("you need an action");
    }

    let conn = Connection::open("./db.sqlite").unwrap();

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                err!("specify a task name");
            }

            conn.execute(
                "CREATE TABLE IF NOT EXISTS task (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
                (),
            )
            .unwrap();
            conn.execute("INSERT INTO task (name) VALUES (?)", [&args[2]])
                .unwrap();
            // tasks.push(&args[2]);
        }

        "remove" => {
            if args.len() < 3 {
                err!("specify which task to remove");
            }

            // if let Ok(ind) = args[2].parse() {
            //     // tasks.remove(ind);
            // } else {
            //     err!("task index not a number");
            // }
        }

        "list" => {
            let mut stmt = conn.prepare("SELECT id, name FROM task").unwrap();
            for row in stmt
                .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
                .unwrap()
            {
                let (id, name): (i32, String) = row.unwrap();
                println!("{id}: {name}");
            }
        }

        _ => {
            err!("unknown command: {}", args[1]);
        }
    }
}
