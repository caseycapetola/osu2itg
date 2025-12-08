use std::fs::File;
use std::io::Read;
use regex::Regex;

// Reads file into string
fn _read_file(file_path: String) -> String {
    let mut f = File::open(file_path).expect("Unable to open file");
    let mut data = String::new();
    f.read_to_string(&mut data).expect("Unable to read data");
    return data;
}

// Splits file by [Sections]
pub fn _parse_file(file_path: String) -> Vec<String> {
    let data = _read_file(file_path);
    let re = Regex::new(r"(\r?\n){2,}").unwrap();
    let sections: Vec<String> = re.split(&data).map(|s| s.to_string()).collect();
    println!("Parsed {} sections from file", sections.len());
    return sections;
}