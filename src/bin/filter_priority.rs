use std::env;
use std::io;

use rusqlite::{Connection, Error};
use serde_json::to_string;
use task_hookrs::import::import_task;
use task_hookrs::task::TW26;
use task_hookrs::task::Task;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Skipping count hook: Note enough args");
        println!("Usage: filter_priority <TASK_DATA_DIR>");

        std::process::exit(0)
    }

    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();
    let db = args[1].to_owned() + "taskchampion.sqlite3";

    let output = parse_priority(&task, &db);
    println!("{}", output);
}

fn parse_priority(task: &str, db: &str) -> String {
    let mut parsed_task: Task<TW26> = import_task(task).expect(task);
    let priority = &parsed_task.priority();
    return match priority.map(String::as_str) {
        Some("T") => {
            if count_priority_t(db).unwrap() >= 3 {
                parsed_task.set_priority(Some("H"));
            }
            return to_string(&parsed_task).unwrap();
        }
        _ => String::from(task),
    };
}

fn count_priority_t(db: &str) -> Result<i32, Error> {
    let conn: Connection = match Connection::open(db) {
        Ok(conn) => conn,
        Err(e) => {
            println!("Skipping count hook: Error connecting to database {db:?} - {e:?}");
            std::process::exit(0)
        }
    };
    let result: i32 = conn.query_row("SELECT COUNT() FROM tasks WHERE (data LIKE '%\"status\":\"pending\"%' OR data LIKE '%\"status\":\"recurring\"%') AND data NOT LIKE '%\"wait\":%' AND data LIKE %\"priority\":\"T\"%", [], |row| row.get(0))?;
    Ok(result)
}
