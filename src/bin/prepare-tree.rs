use std::fmt::{ Error};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;
use rust_sqlite_file_redis::{FilePath, get_file_patch};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oder_as_tree() {
        let lines:Vec<String> = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
        ];
        let mut tree:Vec<String> = vec![];
        build_tree(&mut tree, &lines, lines.len()).expect("tree impossible to build");
        assert_eq!(tree, vec![
            String::from("4"),
            String::from("2"),
            String::from("1"),
            String::from("3"),
            String::from("5"),
        ])
    }

    #[test]
    fn fold() {
        let vec = vec![
            String::from("a"),String::from("b"),String::from("c")
        ];
        let str = vec.join("\n");
        assert_eq!(str,"a\nb\nc");
    }

    #[test]
    fn subarray() {
        assert_eq!([1,2,3][0..=1], [1,2])
    }

    #[test]
    fn usize_max() {
        assert_eq!(usize::MAX, 18446744073709551615);
    }

    #[test]
    fn i_log_16777215() {
        let x:i32 = 16777215;
        assert_eq!(x.ilog2(), 23);
    }

    #[test]
    fn cast_log_16777215() {
        let x:i32 = 16777215;
        assert_eq!((x as f64).log2() as i32, 23);
    }
}

fn build_tree(tree: &mut Vec<String>, lines: &Vec<String>, len: usize) -> Result<(), Error> {
    // println!("lines: {:?}/{:?}/{:?}", tree.len(), lines.len(), len);

    if len == 0 {
        return Ok(());
    }

    let power= (len as f64).log2() as i32;
    // let power= len.ilog2();
    let index = (1 << power) - 1;

    tree.push(format!("{}",lines[index]));

    let sub_lines_1:&Vec<String> = &lines[0..index].try_into().unwrap();
    let sub_lines_2:&Vec<String> = &lines[index+1..len].try_into().unwrap();

    build_tree(tree, sub_lines_1, index).expect("build tree");
    build_tree(tree, sub_lines_2, sub_lines_2.len()).expect("build tree");

    Ok(())
}



fn main() {
    let FilePath {text: file_in_patch, tree: file_out_patch, tree_meta, .. } = get_file_patch(None);
    let file = File::open(file_in_patch).expect("Cant open file");
    let buf = BufReader::new(file).lines();
    let lines = buf.map(|l| l.expect("Could not parse line"))
        .collect::<Vec<String>>();
    let mut tree:Vec<String> = Vec::new();

    // println!("len {:?}", lines.len());
    // println!("a4 {:?}", lines[16777214]);
    // println!("a5 {:?}", lines[16777215]);
    // println!("a6 {:?}", lines[16777216]);

    build_tree(&mut tree, &lines, lines.len()).expect("tree impossible to build");

    // println!("L {:?}", lines);
    // println!("T {:?}", tree);

    let mut out = File::create(file_out_patch).expect("Cannot create tree");
    out.write_all(tree.join("\n").as_bytes()).expect("cannot write data file");
    out.sync_data().expect("Can't sync out data");

    let mut out_meta = File::create(tree_meta).expect("Cannot create tree meta");
    out_meta.write_all(format!("{}", tree.len()).as_bytes()).expect("cannot write meta file");
    out_meta.sync_data().expect("Can't sync meta data");
    // println!("{:?}", buf.next());
}