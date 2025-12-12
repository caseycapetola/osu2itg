pub mod osu;
mod parser;
mod utils;

use std::env;
use std::process;
use crate::parser::osu_to_ssc;

fn print_usage() {
    eprintln!("Usage: osu2itg <input_file.osu>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        print_usage();
        process::exit(1);
    }

    let input_file = &args[1];
    if !input_file.ends_with(".osu") {
        eprintln!("Error: Input file must have a .osu extension.");
        print_usage();
        process::exit(1);
    }

    match osu_to_ssc::osu_to_ssc(input_file) {
        Ok(_) => {
            let output_file = format!("{}.ssc", input_file.trim_end_matches(".osu"));
            println!("Successfully converted to: {}", output_file);
        }
        Err(e) => {
            eprintln!("Error during conversion: {}", e);
            process::exit(1);
        }
    }
}
