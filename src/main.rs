#![feature(plugin)]
#![plugin(rocket_codegen)]

// STANDARD STUFF
extern crate rand;

// WEB FRAMEWORK
extern crate rocket;
// #[macro_use] extern crate rocket_contrib;

// SERDE SERIALIZATION
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod app_structs;
mod json_handling;
mod file_handling;

use json_handling::encode_to_string;
use rocket::State;
use rocket::http::Status;
use rocket::response::content::JSON;
use rocket::response::Failure;
use helpers::{random_number,
              pad,
              get_ref_word_by_id,
              get_vocabulary_by_id};
use app_structs::{VocabularyContext,
                  Vocabulary,
                  Word};

mod helpers {
    use app_structs::{Vocabulary, Word, VocabularyContext};
    use rocket::response::Failure;
    use rocket::http::Status;
    use rand::{thread_rng, Rng};

    pub fn random_number
        (min: u64, max: u64)
         -> u64
    {
        thread_rng().gen_range(min, max)
    }

    pub fn pad
        (number: u64, padding_char: char, padding_length: usize)
         -> String
    {
        let mut padded: String = number.to_string();
        loop {
            if padded.len() >= padding_length {
                break;
            }
            padded = [padding_char.to_string(), padded].join("");
        }
        padded
    }

    pub fn get_ref_word_by_id<'voc_lifetime>
        (voc: &'voc_lifetime mut Vocabulary, word_id: &str)
         -> Result<&'voc_lifetime mut Word, Failure>
    {
        match voc.words.get_mut(word_id) {
            Some(mut the_word) => Ok(the_word),
            _ => return Err(Failure(Status::NotFound))
        }
        //result
    }

    pub fn get_vocabulary_by_id<'voc_ctx_lifetime>
        (voc_ctx: &'voc_ctx_lifetime VocabularyContext,
         voc_id: &str)
         -> Result<&'voc_ctx_lifetime mut Vocabulary, Failure>
    {
        match voc_ctx.vocabularies.get(voc_id) {
            Some(mut voc) => Ok(voc),
            _ => Err(Failure(Status::NotFound))
        }
    }
}



#[get("/learn/<vocabulary_id>/<word_id>/<status>")]
fn learn_word
    (vocabulary_context: State<VocabularyContext>,
     vocabulary_id: &str,
     word_id: &str,
     status: bool)
     -> Result<JSON<String>, Failure>
{
    match get_vocabulary_by_id(&vocabulary_context, &vocabulary_id) {
        Ok(mut voc_ref) => {
            match get_ref_word_by_id(voc_ref, &word_id) {
                Ok(word_ref) => {
                    word_ref.metadata.learned = true;
                    Ok(JSON(serde_json::to_string(word_ref).expect("could not serialize word")))
                },
                _ => Err(Failure(Status::NotFound))
            }
        },
        _ => Err(Failure(Status::NotFound))
    }
}

#[get("/word/<vocabulary_id>/<word_id>")]
fn get_word(
    vocabulary_context: State<VocabularyContext>,
    vocabulary_id: &str,
    word_id: &str
) -> Result<JSON<String>, Failure>
{
    match get_vocabulary_by_id(&vocabulary_context, &vocabulary_id) {
        Ok(mut voc_ref) => match get_ref_word_by_id(&mut voc_ref, &word_id){
            Ok(word_ref) =>
                Ok(JSON(serde_json::to_string(word_ref).expect("could not serialize word"))),
            _ => Err(Failure(Status::NotFound))
        },
        _ => Err(Failure(Status::NotFound))
    }
}

#[get("/word/random/<vocabulary_id>")]
fn get_random_word(
    vocabulary_context: State<VocabularyContext>,
    vocabulary_id: &str
) -> Result<JSON<String>, Failure>
{
    let mut voc_ref: &Vocabulary = match get_vocabulary_by_id(
        &vocabulary_context,
        &vocabulary_id
    ) {
        Ok(voc_ref) => voc_ref,
        _ => return Err(Failure(Status::NotFound))
    };

    let random_word_id: String = pad(
        random_number(0, voc_ref.words.len() as u64),
        '0',
        voc_ref.words.len().to_string().len()
    );

    // with error treatment
    match get_ref_word_by_id(
        &mut voc_ref,
        &random_word_id
    ) {
        Ok(word_id) =>
            Ok(JSON(serde_json::to_string(&word_id)
                    .expect("could not serialize struct Word"))),
        Err(err) => {
            println!("random_word_id: {:?}, err: {:?}", &random_word_id, err);
            Err(Failure(Status::NotFound))
        },
    }
}

#[get("/delete_word/<vocabulary_id>/<word_id>")]
fn delete_word(vocabulary_id: &str, word_id: &str) -> () {
    //
}
#[get("/vocabulary/<vocabulary_id>")]
fn get_vocabulary
    (vocabulary_id: &str, vocabulary_context: State<VocabularyContext>)
     -> Result<JSON<String>, Failure>
{
    match get_vocabulary_by_id(
        &vocabulary_context,
        &vocabulary_id
    ) {
        Ok(voc_ref) => Ok(JSON(serde_json::to_string(voc_ref)
                               .expect("could not serialize a Vocabulary"))),
        _ => Err(Failure(Status::NotFound))
    }
}



fn main() {
    use app_structs::VocabularyContext;

    rocket::ignite()
        .manage(VocabularyContext {..Default::default()})
        .mount("/", routes![get_random_word])
        .mount("/", routes![learn_word])
        .mount("/", routes![get_word])
        .mount("/", routes![delete_word])
        .mount("/", routes![get_vocabulary])
        .launch();
}
