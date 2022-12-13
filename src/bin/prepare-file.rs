use std::env;
use std::fs::File;
use std::io::LineWriter;
use std::io::prelude::*;

fn add_layer(file: &mut LineWriter<File>, alphabet: &Vec<String>, word: &str, size: u8, layer: u8) {
    for letter in alphabet.iter() {
        if size > layer {
            let new_word = format!("{}{}",word, letter);
            add_layer(file, alphabet, &new_word[..], size, layer + 1);
        } else {
            file.write_all((word.to_owned() + letter + "\n").as_bytes()).expect("cannot add lines");
        }
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = if args.len() > 1usize { &args[1] } else { "generated.txt" };
    let size: u8 = if args.len() > 2usize { args[2].parse().unwrap_or(1) } else { 1 };
    let mut alphabet = vec![String::new(); 0];

    println!("{:?}", file_name);
    println!("{:?}", size);
    println!("--- --- ---");

    for i in 10u8..=78u8 {
        let c= (i + 0x30) as char;
        println!("{}, {}", i + 0x30, c);
        alphabet.push(String::from(c));
    }

    println!("{:?}", alphabet);

    let file = File::create(file_name).expect("Cannot create file");
    let mut file = LineWriter::new(file);

    add_layer(&mut file, &alphabet, "", size, 0u8);

    file.flush().expect("Cannot save file");
}