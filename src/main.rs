mod osu_parser;
mod file_tools;

use crate::osu_parser::OsuParser;
use std::env;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut parser = OsuParser::new("assets/Wings_of_Justice/woj_reduced.osu".to_string());
    let file_data = parser.parse_file();

    parser.write_chart(&file_data, "E:\\Projects\\osu2itg\\test.ssc");
}
