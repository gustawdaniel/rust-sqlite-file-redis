use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Read};
use std::path::Path;
use std::string::ToString;
use std::cmp::{Ordering, min};

// const FILE_PATH: &str = "./5.txt";
// const FILE_TREE_PATH: &str = "./5-tree.txt";

// const FILE_PATH: &str = "./collins-head.txt";
// const FILE_TREE_PATH: &str = "./collins-head-tree.txt";

const FILE_PATH: &str = "./collins-scrabble-2019.txt";
const FILE_TREE_PATH: &str = "./collins-scrabble-2019-tree.txt";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_using_text_exists() {
        assert_eq!(find_using_text("hello","./collins-head.txt"), true);
    }

    #[test]
    fn find_using_text_non_exists() {
        assert_eq!(find_using_text("olleh", "./collins-head.txt"), false);
    }

    #[test]
    fn find_using_split_in_lines_1() {
        let buffer = BufReader::new("1".as_bytes());
        let mut lines = buffer.lines();
        let (res, count) = find_using_split_in_lines("1", &mut lines, 5);
        assert_eq!(res, true);
        assert_eq!(count, 1);
    }

    #[test]
    fn find_using_split_in_lines_1_in_5() {
        let buffer = BufReader::new("4\n2\n1\n3\n5".as_bytes());
        let mut lines = buffer.lines();
        let (res, count) = find_using_split_in_lines("1", &mut lines, 5);
        assert_eq!(res, true);
        assert_eq!(count, 3);
    }

    #[test]
    fn find_using_split_in_lines_2_in_5() {
        let buffer = BufReader::new("4\n2\n1\n3\n5".as_bytes());
        let mut lines = buffer.lines();
        let (res, count) = find_using_split_in_lines("2", &mut lines, 5);
        assert_eq!(res, true);
        assert_eq!(count, 2);
    }

    #[test]
    fn find_using_split_in_lines_3_in_5() {
        let buffer = BufReader::new("4\n2\n1\n3\n5".as_bytes());
        let mut lines = buffer.lines();
        let (res, count) = find_using_split_in_lines("3", &mut lines, 5);
        assert_eq!(res, true);
        assert_eq!(count, 3);
    }

    #[test]
    fn find_using_split_in_lines_4_in_5() {
        let buffer = BufReader::new("4\n2\n1\n3\n5".as_bytes());
        let mut lines = buffer.lines();
        let (res, count) = find_using_split_in_lines("4", &mut lines, 5);
        assert_eq!(res, true);
        assert_eq!(count, 1);
    }

    #[test]
    fn find_using_split_in_lines_5_in_5() {
        let buffer = BufReader::new("4\n2\n1\n3\n5".as_bytes());
        let mut lines = buffer.lines();
        let (res, count) = find_using_split_in_lines("5", &mut lines, 5);
        assert_eq!(res, true);
        assert_eq!(count, 2);
    }

    #[test]
    fn find_using_split_in_lines_6_in_5() {
        let buffer = BufReader::new("4\n2\n1\n3\n5".as_bytes());
        let mut lines = buffer.lines();
        let (res, count) = find_using_split_in_lines("6", &mut lines, 5);
        assert_eq!(res, false);
        assert_eq!(count, 2);
    }

    #[test]
    fn find_using_split_in_lines_in_11() {
        let conditions = vec![
            ("h_8", true, 1),
            ("d_4", true, 2),
            ("b_2", true, 3),
            ("a_1", true, 4),
            ("c_3", true, 4),
            ("f_6", true, 3),
            ("e_5", true, 4),
            ("g_7", true, 4),
            ("j_10", true, 2),
            ("i_9", true, 3),
            ("k_11", true, 3),
            ("l_12", false, 3),
        ];

        for (word,should_be_correct, should_be_count) in conditions {
            let text = [
                "h_8","d_4","b_2","a_1","c_3","f_6","e_5","g_7","j_10","i_9","k_11"
            ].join("\n");
            let buffer = BufReader::new(text.as_bytes());
            let mut lines = buffer.lines();
            let (correct, count) = find_using_split_in_lines(word, &mut lines, 11);
            assert_eq!(correct, should_be_correct);
            assert_eq!(count, should_be_count);
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_using_text(word: &str, path: &str) -> bool {
    let word = word.to_uppercase();
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                if line == word {
                    return true;
                }
            }
        }
    }

    false
}

fn find_using_split_in_lines<R: Read>(word: &str, lines: &mut Lines<BufReader<R>>, size: usize) -> (bool, u32) {
    // println!("l1 {:?}", size);
    // println!("l2 {:?}",lines);

    let mut index = 0;
    let mut power = (size as f32).log2() as i32;
    let mut level = power;
    let mut position = 0;
    let mut counter = 1;

    loop {
        let n = &lines.nth(index).unwrap_or(Ok(String::from(""))).unwrap()[..];

        // println!("n [s={}, i={}, p={}, l={}, pos={}, c={}, n={:?}]", size, index, power,level,position,counter, n);

        if n == "" {
            return (false, counter);
        }

        match n.cmp(word) {
            Ordering::Less => {
                index = (1 << power ) - 1;
                // println!("Too small! i={}", index);
            }
            Ordering::Greater => {
                // println!("Too big!");
                index = 0;
            }
            Ordering::Equal => {
                // println!("You win!");
                return (true, counter);
            }
        }
        level -= 1;
        if size-position-1 <= 0 {
            return (false, counter);
        }

        position += index + 1;

        let base = if size - position - 1 > 0 { ((size - position - 1) as f32).log2() as i32 } else { 0 };

        // println!("posi = {}", position);
        // println!("base {}", base);
        // println!("level {}", level);
        power = min(level,base);
        // println!("power {}", power);
        // power -= 1;
        counter += 1;
    }
}

fn find_using_split(word: &str, path: &str) -> bool {
    let f1 = File::open(path).unwrap();
    let f2 = File::open(path).unwrap();
    let b1 = io::BufReader::new(f1);
    let b2 = io::BufReader::new(f2);
    let size = b1.lines().count();
    let mut lines = b2.lines();

    let (res, ..) = find_using_split_in_lines(word, &mut lines, size);

    res
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let method = &env::var("METHOD").unwrap_or("file".to_string())[..];

    println!("{:?}", method == "split");
    println!("{:?}", "split");

    let res: bool = match method {
        "text" => find_using_text(args[1].as_str(), FILE_PATH),
        "split" => find_using_split(args[1].as_str(), FILE_TREE_PATH),
        _ => find_using_text(args[1].as_str(), FILE_PATH)
    };

    println!("{:?}", args);
    println!("{:?}", method);
    println!("{:?}", res);
}
