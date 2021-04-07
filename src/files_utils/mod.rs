use std::fs::File ;
use std::path::Path;
use std::io::{ BufRead, BufReader, Write };
use serde_json::from_str;
use crate::reqs::ResponseApi;


pub fn verify_file(filename: &'static str) -> bool {
    Path::new(filename).exists()
}


pub fn write_file(filename: &'static str, data: String) -> &'static str {
    let mut file = File::create(filename).unwrap();
    file.write_all(data.as_bytes()).unwrap();

    filename
}

pub fn read_file(filename: &'static str) -> ResponseApi {
    let input = File::open(filename).unwrap();
    let read_lines = BufReader::new(input);

    let mut lines = String::new();

    for line in read_lines.lines() {
        lines += line.unwrap().as_str();
    }

    let json: ResponseApi = from_str(&lines).unwrap();

    json
}
