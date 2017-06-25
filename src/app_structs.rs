extern crate serde_json;

use std::collections::HashMap;

use serde_json::map::Map;
use serde_json::value::Value;

use json_handling::parse_from_file;

#[derive(Serialize, Deserialize, Clone)]
pub struct Vocabulary {
    pub metadata: VocabularyMetadata,
    pub words: HashMap<String, Word>
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
    pub vocabularies: HashMap<String, Vocabulary>
}

// Here is a macro for creating some HashMap with initial values.
// This comes in handy when we implement the Default trait for VocabularyContext.
// Got it from: https://stackoverflow.com/a/27582993/1829329
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut a_hashmap = ::std::collections::HashMap::new();
            $(
                a_hashmap.insert($key, $value);
            )+
            a_hashmap
        }
     };
);

impl Default for VocabularyContext {
    fn default () -> VocabularyContext {
        VocabularyContext {
            vocabularies : map!{
                "HSK1".to_string()
                    => parse_from_file("src/data/hsk-1.json".to_string()),
                "HSK2".to_string()
                    => parse_from_file("src/data/hsk-2.json".to_string()),
                "HSK3".to_string()
                    => parse_from_file("src/data/hsk-3.json".to_string()),
                "HSK4".to_string()
                    => parse_from_file("src/data/hsk-4.json".to_string()),
                "HSK5".to_string()
                    => parse_from_file("src/data/hsk-5.json".to_string()),
                "HSK6".to_string()
                    => parse_from_file("src/data/hsk-6.json".to_string())
            }
        }
    }
}
