mod osu_parser;
mod osu_util;
mod file_tools;

use crate::osu_parser::OsuParser;
use std::env;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut parser = OsuParser::new("assets/yomiyori_real/yomiyori.osu".to_string());
    let file_data = parser.parse_file();

    parser.write_chart(&file_data, "E:\\Projects\\osu2itg\\yomiyori.ssc");
}
