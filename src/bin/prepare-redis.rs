use redis::{Commands, Connection};

use std::fs::File;
use std::io::{BufRead, BufReader};
use rust_sqlite_file_redis::{FilePath, get_file_patch};

fn main() {
    let FilePath { text, base, redis_url, .. } = get_file_patch(None);
    let file = File::open(text).expect("Cant open file");
    let reader = BufReader::new(file);

    let client = redis::Client::open(redis_url).expect("Can't connect");
    let mut con: Connection = client.get_connection().expect("Connection error");
    let key = "word:".to_owned() + base.trim();

    for line in reader.lines() {
        let line = line.expect("Problem with reading line");
        let _: () = con.sadd(&key,line).unwrap();
    }
}