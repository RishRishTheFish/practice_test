use std::{collections::HashMap, error::Error, fmt::Display, io::{stdin, stdout, Write}, vec};
use std::fs;

fn ask_question() -> String {
    let mut awnser = String::new();
    stdout().flush().unwrap();
    if let Err(error) = stdin().read_line(&mut awnser) {
        println!("test Error: {error}");
    }
    println!("");
    awnser
}

#[derive(PartialEq, serde::Deserialize, Debug)]
struct Classification {
    name: String,
    parent_classification: Option<Box<Classification>>
}

impl Classification {
    fn new(name: String, parent_classification: Classification) -> Classification {
        Classification {
            name,
            parent_classification: Some(Box::new(parent_classification)),
        }
    }
}
#[derive(serde::Deserialize, Clone)]
struct RawBird {
    name: String,
    commonName: String,
    parentNodes: Vec<String>
}
impl RawBird {
    fn new(name: String, parentNodes: Vec<String>, commonName: String) -> RawBird {
        RawBird {
            name, 
            parentNodes,
            commonName,
        }
    }
}
impl From<RawBird> for Bird {
    fn from(raw: RawBird) -> Self {
        Bird {
            name: raw.name,
            commonName: raw.commonName,
            parentNodes: raw.parentNodes
                .iter()
                .map(|item| Classification {
                     name: item.to_string(), parent_classification: None
                }).collect(),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Bird {
    name: String,
    commonName: String,
    parentNodes: Vec<Classification>
}
impl Bird {
    fn new(name: String, parentNodes: Vec<Classification>, commonName: String) -> Bird {
        Bird {
            name, 
            parentNodes,
            commonName,
        }
    }
}
struct Collection {
    classifications: Vec<Classification>,
    birds: Vec<Bird>
}
impl Collection {
    fn new() -> Collection {
        Collection { classifications: vec![], birds: vec![] }
    }
    fn add_and_check_for_duplicates_classifications(&mut self, classification: Classification) -> Option<usize> {
        if !self.classifications.iter().any(|c| *c == classification) {
            self.classifications.push(classification);
            Some(self.classifications.len())
        } else {
            None
        }
    }
    fn add_and_check_for_duplicates_bird(&mut self, bird: Bird) -> Option<usize>{
            if !self.birds.iter().any(|b| *b == bird) {
                self.birds.push(bird);
                Some(self.classifications.len())
            } else {
                None
            }
    }
}


fn main() {
    let mut collection = Collection::new();
    //collection.add_and_check_for_duplicates_bird(Bird::new("Test".to_owned(), vec![]));
    let raw_bird_data: String  = fs::read_to_string("C:\\Users\\DuttR\\school\\practice_test\\src\\birdData.json").expect("Failed");
    let raw_bird_json: Vec<RawBird> = serde_json::from_str(&raw_bird_data).expect("Incorrect json");
    let final_bird_json: Vec<Bird> = raw_bird_json.iter().map(|bird| Bird::from(bird.clone())).collect();
    println!("{:#?}", final_bird_json)
}