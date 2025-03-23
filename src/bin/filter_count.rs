use rusqlite::{Connection, Error, params};
use std::env;
use std::fs::File;
use std::io::{ErrorKind, Write};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Skipping count hook: Task data dir not provided");
        std::process::exit(0)
    }
    let db = args[1].to_owned() + "taskchampion.sqlite3";
    let conn: Connection = match Connection::open(&db) {
        Ok(conn) => conn,
        Err(e) => {
            println!("Skipping count hook: Error connecting to database {db:?} - {e:?}");
            std::process::exit(0)
        }
    };
    let queries = vec![
        ("%\"priority\":\"T\"%", args[1].to_owned() + "/.priority"),
        ("%\"tag_in\"%", args[1].to_owned() + "/.inbox"),
    ];
    for (query, file_str) in queries {
        match count_task_instances(&conn, query) {
            Ok(count) => write_count(count, &file_str),
            Err(e) => println!("Skipping count hook: Query error in {db:?}: {query:?} - {e:?}"),
        };
    }
}

fn count_task_instances(conn: &Connection, param: &str) -> Result<i32, Error> {
    let result: i32 = conn.query_row("SELECT COUNT() FROM tasks WHERE (data LIKE '%\"status\":\"pending\"%' OR data LIKE '%\"status\":\"recurring\"%') AND data NOT LIKE '%\"wait\":%' AND data LIKE ?1", params![param], |row| row.get(0))?;
    Ok(result)
}

fn write_count(count: i32, file_str: &str) {
    if count > 0 {
        let mut file = File::create(file_str).unwrap();
        match writeln!(file, "{:?}", count) {
            Ok(_) => (),
            Err(e) => println!("Skipping count hook: Unable to write file {file_str:?} - {e:?}"),
        }
    } else {
        match std::fs::remove_file(file_str) {
            Ok(_) => (),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => (),
                err => {
                    println!("Skipping count hook: Unable to remove file {file_str:?} - {err:?}")
                }
            },
        }
    }
}
