mod osu_parser;

// use std::fs::File;
use crate::osu_parser::OsuParser;

fn main() {
    let buf = Vec::new();
    let mut parser = OsuParser::new("assets/Wings_of_Justice/woj.osu".to_string(), buf, 0,);
    println!("Hello, world!");
    println!("File: {:?}", parser.get_file());
    println!("File contents: {:?}", parser.read_file());
}
