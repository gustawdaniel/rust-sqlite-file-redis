use std::fs::File;
use std::io::{BufRead, BufReader};
use rust_sqlite_file_redis::{FilePath, get_file_patch};
use rusqlite::{Connection};

fn main() {
    let FilePath { text, sqlite, .. } = get_file_patch(None);
    let file = File::open(text).expect("Cant open file");
    let reader = BufReader::new(file);

    let mut conn = Connection::open(sqlite)
        .expect("Can't connect");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS words (
             word text not null primary key
         )",
        [],
    ).expect("Cant create table");

    let tx = conn.transaction().expect("Can't open transaction");

    tx.execute("delete from words", []).expect("Can't delete");

    for line in reader.lines() {
        let line = line.expect("Problem with reading line");
        tx.execute(
            "insert into words (word) values (?1)",
            [line],
        ).expect("Can't insert");
    }

    tx.commit().expect("Can't commit");
}