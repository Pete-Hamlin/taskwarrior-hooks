use rusqlite::{Connection, Error, params};
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Task data dir not provided - skipping");
        std::process::exit(0)
    }
    let db = format!("{:?}/taskchampion.sqlite3", args[1]);
    let conn: Connection = match Connection::open(&db) {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error connecting to database {:?} - {:?}", db, e);
            std::process::exit(0)
        }
    };
    let queries = vec![
        ("%\"priority\":\"T\"%", format!("{:?}/.priority", args[1])),
        ("%\"tag_in\"%", format!("{:?}/.inbox", args[1])),
    ];
    for (query, file_str) in queries {
        match count_task_instances(&conn, query) {
            Ok(count) => write_count(count, &file_str),
            Err(e) => println!("Error with query {:?}: {:?}", query, e),
        };
    }
}

fn count_task_instances(conn: &Connection, param: &str) -> Result<i32, Error> {
    let result: i32 = conn.query_row("SELECT COUNT() FROM tasks WHERE data LIKE '%\"status\":\"pending\"%' AND data NOT LIKE '%\"wait\":%' AND data LIKE ?1", params![param], |row| row.get(0))?;
    Ok(result)
}

fn write_count(count: i32, file_str: &str) {
    println!("Got count {:?}", count);
    let mut file = File::create(file_str).unwrap();
    writeln!(file, "{:?}", count).expect(&format!("Unable to write file {:?}", file_str));
}
