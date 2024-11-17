// osu!std file parser

use std::{fs::File, io::Read};

pub struct OsuParser {
    file: String,
    buffer: Vec<u8>,
    cursor: usize,
}

impl OsuParser {
    pub fn new(file: String, buffer: Vec<u8>, cursor: usize) -> Self {
        OsuParser {
            file,
            buffer,
            cursor,
        }
    }

    pub fn get_file(&self) -> String {
        self.file.clone()
    }

    pub fn get_buffer(&self) -> &Vec<u8> {
        &self.buffer
    }

    pub fn get_cursor(&self) -> usize {
        self.cursor
    } 

    pub fn read_file(&mut self) -> String {
        let mut f = File::open(self.file.clone()).expect("Unable to open file");
        let mut data = String::new();
        f.read_to_string(&mut data).expect("Unable to read data");
        return data;
    }
}