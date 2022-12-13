use rust_sqlite_file_redis::{get_file_patch, FilePath};
use std::fs::{File};
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;
use std::os::unix::fs::FileExt;

fn main() {
    let FilePath { tree, bin, .. } = get_file_patch(None);
    // println!("TX {}", text);
    // println!("TR {}", tree);
    // println!("BI {}", bin);

    let file = File::open(tree).expect("Can't read file");
    let reader = BufReader::new(file);

    let mut lines:Vec<String> = vec![];
    let mut max_length_word:usize = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                lines.push(line.clone());
                match max_length_word.cmp(&line.len()) {
                    Ordering::Less => { max_length_word = *&line.len() },
                    Ordering::Greater => {},
                    Ordering::Equal => {},
                }
            }
            Err(_err) => {}
        }

    }

    let bin_file = File::create(bin).expect("Can't create");
    let h = bin_file.write_at(format!("h=0x{:08x},w=0x{:02x}\n", lines.len(), max_length_word).as_bytes(),0).expect("Cannot write buffer");

    let mut next = String::from("");
    for line in lines {
        next += &*format!("{:<1$}", line, max_length_word);
    }

    bin_file.write_at(next.as_bytes(), h as u64).expect("Cannot write buffer");
    bin_file.sync_data().expect("Can't sync data");
}