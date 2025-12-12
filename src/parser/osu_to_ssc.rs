use std::io;

// Core function to convert an osu! file to SSC format
use crate::parser::osu_parser::OsuParserV2;

// Return output path or error
pub fn osu_to_ssc(input_path: &str) -> io::Result<()> {
    let parser_v2 = OsuParserV2::new(input_path.to_string());
    let output_path = format!("{}.ssc", input_path.trim_end_matches(".osu"));
    parser_v2.write_chart(&output_path);

    Ok(())

}