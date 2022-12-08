use std::fmt::{ Error};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;

// const  FILE_IN_PATCH: &str = "./5.txt";
// const  FILE_OUT_PATCH: &str = "./5-tree.txt";

// const  FILE_IN_PATCH: &str = "./collins-head.txt";
// const  FILE_OUT_PATCH: &str = "./collins-head-tree.txt";

const  FILE_IN_PATCH: &str = "./collins-scrabble-2019.txt";
const  FILE_OUT_PATCH: &str = "./collins-scrabble-2019-tree.txt";

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
}

fn build_tree(tree: &mut Vec<String>, lines: &Vec<String>, len: usize) -> Result<(), Error> {


    if len == 0 {
        return Ok(());
    }

    let power= len.ilog2();
    let index = (1 << power) - 1;
    // println!("P {:?}", power);
    // println!("I {:?}", index);


    tree.push(format!("{}",lines[index]));

    let sub_lines_1:&Vec<String> = &lines[0..index].try_into().unwrap();
    let sub_lines_2:&Vec<String> = &lines[index+1..len].try_into().unwrap();
    // println!("sub_lines_1: {:?}",sub_lines_1);
    // println!("sub_lines_2: {:?}",sub_lines_2);
    build_tree(tree, sub_lines_1, index).expect("build tree");
    build_tree(tree, sub_lines_2, sub_lines_2.len()).expect("build tree");

    Ok(())
}

fn main() {
    let file = File::open(FILE_IN_PATCH).expect("Cant open file");
    let buf = BufReader::new(file).lines();
    let lines = buf.map(|l| l.expect("Could not parse line"))
        .collect::<Vec<String>>();
    let mut tree:Vec<String> = Vec::new();

    build_tree(&mut tree, &lines, lines.len()).expect("tree impossible to build");

    // println!("len {:?}", lines.len());
    // println!("L {:?}", lines);
    // println!("T {:?}", tree);

    let mut out = File::create(FILE_OUT_PATCH).expect("Cannot create");
    out.write_all(tree.join("\n").as_bytes()).expect("cannot write");

    // println!("{:?}", buf.next());
}