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

#[derive(PartialEq, serde::Deserialize, Debug, Clone)]
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
struct Collection<'a> {
    classifications: Vec<&'a Classification>,
    birds: Vec<Bird>
}
impl Collection<'_> {
    fn new<'a>() -> Collection<'a> {
        Collection { classifications: vec![], birds: vec![] }
    }
    fn add_or_return_classification_ref<'a>(&'a mut self, classification: &'static Classification) -> &Classification {
        if !self.classifications.iter().any(|c| **c == *classification) {
            self.classifications.push(&classification);
        }
        self.classifications
            .iter()
            .find(|c| c.name == classification.name)
            .unwrap()
    }
    fn add_and_check_for_duplicates_classifications<'a>(&'a mut self, classification: &'static Classification) -> Option<usize> {
        if !self.classifications.iter().any(|c| c.name == classification.name) {
            self.classifications.push(&classification);
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
    let raw_bird_data: String  = fs::read_to_string("C:\\Users\\DuttR\\school\\practice_test\\src\\birdData.json").expect("Failed");
    let raw_bird_json: Vec<RawBird> = serde_json::from_str(&raw_bird_data).expect("Incorrect json");
    let final_bird_json: Vec<Bird> = raw_bird_json.iter().map(|bird| Bird::from(bird.clone())).collect();
    
    loop {
        println!("Welcome to the Bird Collection!");
        println!("###############################");
        println!("(a) Addes a bird");
        println!("(p) Prints out all birds");
        println!("(c) prints out all classes");
        println!("(t) Adds a bird to a class");
        println!("(k) Adds a class");
        println!("(q) Quits");
        let operation = ask_question();
        match operation.to_lowercase().as_str().trim() {
            "a" => {
                println!("Whats the name of the bird do you want to add?");
                let bird_name = ask_question();
                println!("Whats the scientific name of the bird?");
                let bird_common_name = ask_question();
                collection.birds.push(
                    Bird { name: bird_name, commonName: bird_common_name, parentNodes: vec![] }
                );
            },
            "p" => {
                for bird in &final_bird_json {
                    println!("name: {}, common name: {}, species: {}", bird.name, bird.commonName, 
                        String::from_iter(
                            bird.parentNodes.iter().map(|node| node.name.clone() + " ")
                        ))
                }
            },
            "c" => {
                for classification in final_bird_json.iter().fold(
                    vec![], |mut acc, bird| {
                        let unique_refrences: Vec<Classification> = bird.parentNodes
                            .iter()
                            .filter(|unique_classificaion|
                                !acc.iter().any(|matching_classification: &Classification| matching_classification == *unique_classificaion)
                                )
                                .cloned()
                                .collect();
                            acc.extend(unique_refrences);
                            acc
                    }).into_iter().map(|classification| classification.name.clone()){
                    println!("{}", classification)
                }
            },
            "t" => {},
            "k" => {},
            "q" => break,
            _ => {
                println!("Non-existent operation")
            }
        }
    }
}