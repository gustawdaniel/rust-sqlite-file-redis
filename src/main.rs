use std::{env, usize};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Read, Seek, SeekFrom};
use std::path::Path;
use std::string::ToString;
use std::cmp::{Ordering, min};
use rust_sqlite_file_redis::{FilePath, get_file_patch};
use memmap::Mmap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_using_text_exists() {
        assert_eq!(find_using_text("hello", String::from("./collins-head.txt")), true);
    }

    #[test]
    fn find_using_text_non_exists() {
        assert_eq!(find_using_text("olleh", String::from("./collins-head.txt")), false);
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

        for (word, should_be_correct, should_be_count) in conditions {
            let text = [
                "h_8", "d_4", "b_2", "a_1", "c_3", "f_6", "e_5", "g_7", "j_10", "i_9", "k_11"
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

fn find_using_text(word: &str, path: String) -> bool {
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
    let mut index = 0;
    let mut power = (size as f64).log2() as i32;
    let mut level = power;
    let mut position = 0;
    let mut counter = 1;

    loop {
        let n = &lines.nth(index).unwrap_or(Ok(String::from(""))).unwrap()[..];
        if n == "" { return (false, counter); }
        match n.cmp(word) {
            Ordering::Less => { index = (1 << power) - 1; }
            Ordering::Greater => { index = 0; }
            Ordering::Equal => { return (true, counter); }
        }
        level -= 1;
        if size - position - 1 <= 0 { return (false, counter); }
        position += index + 1;
        let base = if size - position - 1 > 0 { ((size - position - 1) as f64).log2() as i32 } else { 0 };
        power = min(level, base);
        counter += 1;
    }
}

fn find_using_bin_in_file(word: &str, mut file: &File, w: usize, h: usize) -> (bool, u32) {
    let mut index = 0;
    let mut power = (h as f64).log2() as i32;
    let mut level = power;
    let mut position = 0;
    let mut counter = 1;

    loop {
        let mut s = String::new();
        file.seek(SeekFrom::Current((w * index) as i64)).expect("Can't seek");
        file.take(w as u64).read_to_string(&mut s).expect("Can't read as string");

        let n = &s.trim()[..];
        if n == "" { return (false, counter); }
        match n.cmp(word) {
            Ordering::Less => { index = (1 << power) - 1; }
            Ordering::Equal => { return (true, counter); }
            Ordering::Greater => { index = 0; }
        }
        level -= 1;
        if h - position - 1 <= 0 { return (false, counter); }
        position += index + 1;
        let base = if h - position - 1 > 0 { ((h - position - 1) as f64).log2() as i32 } else { 0 };
        power = min(level, base);
        counter += 1;
    }
}

fn find_using_split(word: &str, path: String, meta: String) -> bool {
    let f_meta = File::open(meta.clone()).unwrap();
    let f_data = File::open(path).unwrap();
    let b_meta = io::BufReader::new(f_meta);
    let b_data = io::BufReader::new(f_data);
    let size: usize = b_meta.lines().next().unwrap().expect("can't unwrap").parse().expect("can't find lines size");
    let mut lines = b_data.lines();

    let (res, ..) = find_using_split_in_lines(word, &mut lines, size);

    res
}

fn find_using_bin(word: &str, path: String) -> bool {
    let mut file = File::open(path).unwrap();
    file.seek(SeekFrom::Current(4)).expect("Can't seek");

    let mut s = String::new();
    let mut reader = file.take(8);
    reader.read_to_string(&mut s).expect("Can't read as string");

    let h: usize = usize::from_str_radix(&*s, 16).unwrap();
    let mut file = reader.get_ref();

    s = String::new();
    file.seek(SeekFrom::Current(5)).expect("Can't seek");
    file.take(2).read_to_string(&mut s).expect("Can't read as string");

    let w: usize = usize::from_str_radix(&*s, 16).unwrap();
    file.seek(SeekFrom::Current(1)).expect("Can't seek");

    let (res, ..) = find_using_bin_in_file(word, &mut file, w, h);
    res
}

fn find_using_bin_in_map(word: &str, map: &Mmap, w: usize, h: usize, offset: usize) -> (bool, u32) {
    let mut power = (h as f64).log2() as i32;
    let mut level = power;
    let mut position = 0;
    let mut counter = 1;

    loop {
        let a = offset + (position * w);
        let b = offset + ((position + 1) * w);
        let s = String::from_utf8(map[a..b].to_vec()).expect("Cannot unwrap");

        let n = &s.trim()[..];
        if n == "" { return (false, counter); }
        let index = match n.cmp(word) {
            Ordering::Greater => { 0 }
            Ordering::Less => { (1 << power) - 1 }
            Ordering::Equal => { return (true, counter); }
        };
        level -= 1;
        if h - position - 1 <= 0 { return (false, counter); }
        position += index + 1;
        let base = if h - position - 1 > 0 { ((h - position - 1) as f64).log2() as i32 } else { 0 };
        power = min(level, base);
        counter += 1;
    }
}

fn find_using_mem(word: &str, path: String) -> bool {
    let file = File::open(path).expect("Can't open file");
    let map = unsafe { Mmap::map(&file).expect("Can't map file") };

    let mut s = String::from_utf8(map[4..12].to_vec()).unwrap();
    let h: usize = usize::from_str_radix(&*s, 16).unwrap();
    s = String::from_utf8(map[17..19].to_vec()).unwrap();
    let w: usize = usize::from_str_radix(&*s, 16).unwrap();

    let (res, ..) = find_using_bin_in_map(word, &map, w, h, 20);
    res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let method = &env::var("METHOD").unwrap_or("file".to_string())[..];

    let FilePath { text: file_path, tree: file_tree_path, bin: file_bin_path, tree_meta } = get_file_patch(Some(2usize));

    let res: bool = match method {
        "text" => find_using_text(args[1].as_str(), file_path),
        "split" => find_using_split(args[1].as_str(), file_tree_path, tree_meta),
        "bin" => find_using_bin(args[1].as_str(), file_bin_path),
        "mem" => find_using_mem(args[1].as_str(), file_bin_path),
        _ => find_using_text(args[1].as_str(), file_path)
    };

    println!("{:?}", method);
    println!("{:?}", res);
}
