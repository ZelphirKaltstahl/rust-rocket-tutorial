#![feature(plugin)]
#![plugin(rocket_codegen)]

// WEB FRAMEWORK
extern crate rocket;
#[macro_use] extern crate rocket_contrib;

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
    use app_structs::{Vocabulary, VocabularyContext, Word};
    use json_handling::encode_to_string;
    use serde_json;
    use rocket::State;
    use rocket::http::Status;
    use rocket::response::Responder;
    use rocket::response::content::JSON;
    use rocket::response::Failure;
    use rocket_contrib::Value;
    use rocket::Outcome;

    #[get("/learn/<vocabulary_id>/<word_id>/<status>")]
    fn learn_word(vocabulary_id: &str, word_id: &str, status: bool) -> () {
        //
    }

    #[get("/word/<vocabulary_id>/<word_id>")]
    fn get_word(vocabulary_context: State<VocabularyContext>,
                vocabulary_id: &str,
                word_id: &str) -> Result<JSON<String>, Failure> {
        let the_vocabularies: Vec<Vocabulary> = vocabulary_context.vocabularies.iter()
            .filter(|voc| voc.metadata.identifier == vocabulary_id)
            .cloned()
            .collect::<Vec<Vocabulary>>();

        if the_vocabularies.is_empty() {
            return Err(Failure(Status::NotFound));
        }

        let the_words: Vec<Word> = the_vocabularies.first().unwrap().words.iter()
            .filter(|word| word.metadata.id == word_id)
            .cloned()
            .collect::<Vec<Word>>();

        if the_words.is_empty() {
            return Err(Failure(Status::NotFound));
        }

        let the_word_as_string: String = serde_json::to_string(&the_words[0])
            .expect("could not serialize struct Vocabulary");

        Ok(JSON(the_word_as_string))
    }

    #[get("/word/next/<vocabulary_id>")]
    fn get_next_word(vocabulary_id: &str) -> () {
        //
    }
    #[get("/word/random/<vocabulary_id>")]
    fn get_random_word(vocabulary_id: &str) -> () {
        //
    }
    #[get("/delete_word/<vocabulary_id>/<word_id>")]
    fn delete_word(vocabulary_id: &str, word_id: &str) -> () {
        //
    }
    #[get("/vocabulary/<vocabulary_id>")]
    fn get_vocabulary(vocabulary_id: &str, vocabulary_context: State<VocabularyContext>) -> String {
        let result: Vec<String> = vocabulary_context.vocabularies.iter()
            .filter(|voc| voc.metadata.identifier.as_str().contains(vocabulary_id))
            .map(encode_to_string)
            .collect::<Vec<String>>();
        result.join("\n\n")
    }
}

mod app_structs {
    use json_handling::parse_from_file;

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Vocabulary {
        pub metadata: VocabularyMetadata,
        pub words: Vec<Word>
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct VocabularyMetadata {
        pub identifier: String,
        pub learned_percentage: u8,
        pub count: u32,
        pub source_note: String
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Word {
        pub metadata: WordMetadata,
        pub translation_data: WordTranslationData
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct WordMetadata {
        pub id: String,
        pub learned: bool,
        pub description: String
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct WordTranslationData {
        pub english: String,
        pub pinyin_numbered: String,
        pub pinyin: String,
        pub simplified: String,
        pub traditional: String
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct VocabularyContext {
        pub vocabularies: Vec<Vocabulary>
    }

    impl Default for VocabularyContext {
        fn default () -> VocabularyContext {
            VocabularyContext {
                vocabularies : vec![
                    parse_from_file("data/hsk-1.json".to_string()),
                    parse_from_file("data/hsk-2.json".to_string()),
                    parse_from_file("data/hsk-3.json".to_string()),
                    parse_from_file("data/hsk-4.json".to_string()),
                    parse_from_file("data/hsk-5.json".to_string()),
                    parse_from_file("data/hsk-6.json".to_string())
                ]
            }
        }
    }
    // impl Clone for VocabularyContext {
    //     fn clone(&self) -> VocabularyContext { *self }
    // }
}

fn main() {
    use app_structs::VocabularyContext;

    rocket::ignite()
        .manage(VocabularyContext {..Default::default()})
        .mount("/", routes![app_routes::learn_word])
        .mount("/", routes![app_routes::get_word])
        .mount("/", routes![app_routes::delete_word])
        .mount("/", routes![app_routes::get_vocabulary])
        .launch();
}
