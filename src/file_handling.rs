use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn read_file(file_path: String) -> String {
    let mut string_data = String::new();
    let mut file = File::open(file_path).expect("Could not open file!");
    file.read_to_string(&mut string_data).expect("Could not read to String!");
    string_data
}

pub fn write_file(file_path: String, data: String) {
    let mut file = File::open(file_path).expect("Could not open file!");
    file.write_all(data.as_bytes()).expect("Could not write to file!");
}
