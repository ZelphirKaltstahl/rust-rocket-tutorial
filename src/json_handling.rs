use serde_json;
use app_structs::Vocabulary;

// TODO: use std::path::Path; // should I use paths?
use file_handling::{read_file, write_file};

pub fn parse_from_string(a_vocabulary_json_string: String) -> Vocabulary {
    let a_vocabulary: Vocabulary = serde_json::from_str(&a_vocabulary_json_string)
        .expect("could not parse json to struct Vocabulary");
    a_vocabulary
}
pub fn encode_to_string(a_vocabulary: &Vocabulary) -> String {
    let encoded_vocabulary = serde_json::to_string(&a_vocabulary)
        .expect("could not serialize struct Vocabulary");
    encoded_vocabulary
}
pub fn parse_from_file(a_file_path: String) -> Vocabulary {
    parse_from_string(read_file(a_file_path))
}
pub fn encode_to_file(a_file_path: String, a_vocabulary: &Vocabulary) {
    write_file(a_file_path, encode_to_string(a_vocabulary))
}
