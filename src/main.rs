// #![feature(iter_advance_by)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string::ToString;

const FILE_PATH: &str = "./collins-head.txt";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_using_text_exists() {
        assert_eq!(find_using_text("hello"), true);
    }

    #[test]
    fn find_using_text_non_exists() {
        assert_eq!(find_using_text("olleh"), false);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_using_text(word: &str) -> bool {
    let word = word.to_uppercase();
    if let Ok(lines) = read_lines(FILE_PATH) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line == word {
                    return true
                }
            }
        }
    }

    false
}

fn find_using_split(_word: &str) -> bool {
    let file = File::open(FILE_PATH).unwrap();
    let mut buffer = io::BufReader::new(file);
    let mut lines = buffer.lines();

    let s = lines.nth(3).expect("No line");

    // let mut lines = buffer.lines();
    // let f2 = buffer.consume(400);

    // println!("c {:?}",buffer.capacity());
    // let mut line = String::new();
    // buffer.read_line(&mut line).expect("Fail to read line");

    // println!("b {:?}",buffer);

    // println!("b {:?}", line);
    // println!("lines {:?}",lines.count());
    println!("s {:?}",s);

    // lines.advance_by(2).expect("TODO: panic message");
    println!("s {:?}",lines.nth(3).expect("No line"));
    // println!("l {:?}",f2);

    true
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let method = &env::var("METHOD").unwrap_or("file".to_string())[..];

    println!("{:?}", method == "split");
    println!("{:?}", "split");

    let res:bool = match method  {
        "text" => find_using_text(args[1].as_str()),
        "split" => find_using_split(args[1].as_str()),
        _ => find_using_text(args[1].as_str())
    };

    println!("{:?}", args);
    println!("{:?}", method);
    println!("{:?}", res);
}
