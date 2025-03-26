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

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::fs;

    fn get_test_db_connection() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("CREATE TABLE tasks (data TEXT NOT NULL)", [])
            .unwrap();
        conn.execute(
            "INSERT INTO tasks (data) VALUES ('{\"status\":\"pending\",\"priority\":\"T\"}')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO tasks (data) VALUES ('{\"status\":\"recurring\",\"priority\":\"T\"}')",
            [],
        )
        .unwrap();
        conn.execute(
                "INSERT INTO tasks (data) VALUES ('{\"status\":\"completed\",\"priority\":\"L\", \"tag_in\": 1}')",
                [],
            )
                .unwrap();
        conn.execute(
            "INSERT INTO tasks (data) VALUES ('{\"status\":\"pending\" \"tag_in\": 1}')",
            [],
        )
        .unwrap();
        conn
    }

    #[test]

    fn test_count_task_t_priority() {
        let conn = get_test_db_connection();

        let count = count_task_instances(&conn, "%\"priority\":\"T\"%").unwrap();
        assert!(count == 2); // Ensure the count is non-negative
    }
    #[test]
    fn test_count_task_inbox() {
        let conn = get_test_db_connection();

        let count = count_task_instances(&conn, "%\"tag_in\"%").unwrap();
        assert!(count == 1); // Ensure the count is non-negative
    }

    #[test]
    fn test_write_count() {
        let temp_file = "test_count_file";
        write_count(5, temp_file);
        let content = fs::read_to_string(temp_file).unwrap();
        assert_eq!(content.trim(), "5");

        write_count(0, temp_file);
        assert!(!fs::metadata(temp_file).is_ok());
    }
}
