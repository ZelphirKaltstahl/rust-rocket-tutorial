#![feature(plugin)]
#![plugin(rocket_codegen)]

// WEB FRAMEWORK
extern crate rocket;

// SERDE SERIALIZATION
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod file_handling {
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
}

mod json_handling {
    use serde_json;
    use app_structs::Vocabulary;
    // use std::path::Path; // should I use paths?
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
}


mod app_routes {
    use app_structs::{Vocabulary, VocabularyContext};
    use json_handling::{parse_from_file, encode_to_string};
    use rocket::State;

    #[get("/learn/<vocabulary_id>/<word_id>/<status>")]
    fn learn_word(vocabulary_id: &str, word_id: &str, status: bool) -> () {
        //
    }
    #[get("/next_word/<vocabulary_id>")]
    fn get_word(vocabulary_id: &str) -> String {
        "Hello World!".to_string()
    }
    #[get("/delete_word/<vocabulary_id>/<word_id>")]
    fn delete_word(vocabulary_id: &str, word_id: &str) -> () {
        //
    }
    #[get("/vocabulary/<vocabulary_id>")]
    fn get_vocabulary(vocabulary_id: &str) -> String {
        "My Vocabulary".to_string()
    }

    #[get("/get_me_a_json")]
    fn get_me_a_json(vocabulary_context: State<VocabularyContext>) -> String {
        let encoded_vocabulary: String = encode_to_string(&vocabulary_context.hsk1);
        encoded_vocabulary
    }
}

mod app_structs {
    use json_handling::parse_from_file;

    #[derive(Serialize, Deserialize)]
    pub struct Vocabulary {
        metadata: VocabularyMetadata,
        words: Vec<Word>
    }

    #[derive(Serialize, Deserialize)]
    pub struct VocabularyMetadata {
        identifier: String,
        learned_percentage: u8,
        count: u32,
        source_note: String
    }

    #[derive(Serialize, Deserialize)]
    pub struct Word {
        metadata: WordMetadata,
        translation_data: WordTranslationData
    }

    #[derive(Serialize, Deserialize)]
    pub struct WordMetadata {
        id: String,
        learned: bool,
        description: String
    }

    #[derive(Serialize, Deserialize)]
    pub struct WordTranslationData {
        english: String,
        pinyin_numbered: String,
        pinyin: String,
        simplified: String,
        traditional: String
    }

    #[derive(Serialize, Deserialize)]
    pub struct VocabularyContext {
        pub hsk1: Vocabulary,
        pub hsk2: Vocabulary,
        pub hsk3: Vocabulary,
        pub hsk4: Vocabulary,
        pub hsk5: Vocabulary,
        pub hsk6: Vocabulary,
    }

    impl Default for VocabularyContext {
        fn default () -> VocabularyContext {
            VocabularyContext {
                hsk1 : parse_from_file("data/hsk-1.json".to_string()),
                hsk2 : parse_from_file("data/hsk-2.json".to_string()),
                hsk3 : parse_from_file("data/hsk-3.json".to_string()),
                hsk4 : parse_from_file("data/hsk-4.json".to_string()),
                hsk5 : parse_from_file("data/hsk-5.json".to_string()),
                hsk6 : parse_from_file("data/hsk-6.json".to_string()),
            }
        }
    }
}

fn main() {
    use app_structs::VocabularyContext;

    rocket::ignite()
        .manage(VocabularyContext {..Default::default()})
        .mount("/", routes![app_routes::learn_word])
        .mount("/", routes![app_routes::get_word])
        .mount("/", routes![app_routes::delete_word])
        .mount("/", routes![app_routes::get_vocabulary])
        .mount("/", routes![app_routes::get_me_a_json])
        .launch();
}
