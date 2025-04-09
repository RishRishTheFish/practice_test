use std::{collections::HashMap, error::Error, fmt::Display, io::{stdin, stdout, Write}, vec};
use std::fs;

fn ask_question() -> String {
    // Constructs a new string for the awnser to be put into
    let mut awnser = String::new();
    // Flush stdout so previous input is not detected
    stdout().flush().unwrap();
    // Reads the actual line (from intput) and also provide error handling
    if let Err(error) = stdin().read_line(&mut awnser) {
        println!("test Error: {error}");
    }
    println!("");
    awnser
}

// Derive macros so i can compare classifications with other ones, debug it, and clone the data
// to avoid memory issues (since preformance loss is acceptable)
#[derive(PartialEq, Debug, Clone)]
// Classification will take a simple string as a name, and a optional, parent classification which is 
// optional for the sake of orphan nodes and when a node is first initilized
// and a box for classification so the data can be consistently borrowed as mutable
struct Classification {
    name: String,
    parent_classification: Option<Box<Classification>>
}

// Implimentation of Classification which provides a constructer funnction
impl Classification {
    fn new(name: String, parent_classification: Classification) -> Classification {
        Classification {
            name,
            parent_classification: Some(Box::new(parent_classification)),
        }
    }
}

// RawBird will be what will be immediately derived from the json file
// as deserialization will not be possible for parentNodes which is a property of Bird with the properties I want
// as in, a classification which is supposed to be a parent node, will have a name and a optional refrence to other parent nodes
// which is not possible with raw json. 
#[derive(serde::Deserialize, Clone)]
struct RawBird {
    name: String,
    commonName: String,
    parentNodes: Vec<String>
}

// A implimentation for RawBird that provides a simple constructer function
impl RawBird {
    fn new(name: String, parentNodes: Vec<String>, commonName: String) -> RawBird {
        RawBird {
            name, 
            parentNodes,
            commonName,
        }
    }
}

// here is a From(RawBird) implimentation to convert the raw json struct into a normal bird struct
// with the only diffrence being that parent nodes will be turned from simple strings into classifications
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

//Macros for PartialEq and Debug for equating a bird with another and printing it out
#[derive(PartialEq, Debug)]
struct Bird {
    name: String,
    commonName: String,
    parentNodes: Vec<Classification>
}
// Basic implimentation of bird with a constructer function
impl Bird {
    fn new(name: String, parentNodes: Vec<Classification>, commonName: String) -> Bird {
        Bird {
            name, 
            parentNodes,
            commonName,
        }
    }
}

// Collection is everything contained in a single struct, this makes it easier to refer to the data
// and to better originize certain functions
struct Collection<'a> {
    classifications: Vec<&'a Classification>,
    birds: Vec<Bird>
}
// The implimentation for Collection which will provide a constructer function, while also providing functions
// for ensuring there are no duplicate classifications, or birds, and one where it will add a classification if none exists,
// and return the refrence (regardless if it existed prior or not)
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
    // Main function, it will run all my code, first thing it does is that it initilizes the collection
    // Then using fs it will read the birdData json file, (with error handling), captured as a string
    // next it is deserilized into Vec<RawBird> (vector of rawbirds) because rawbirds is more accurate to the JSON data
    // then it is converted into a final struct, by iterating over all the birds gained from the json file.
    // using Bird::from it will convert it into regular birds 
    let mut collection = Collection::new();
    let raw_bird_data: String  = fs::read_to_string("C:\\Users\\DuttR\\school\\practice_test\\src\\birdData.json").expect("Failed");
    let raw_bird_json: Vec<RawBird> = serde_json::from_str(&raw_bird_data).expect("Incorrect json");
    let final_bird_json: Vec<Bird> = raw_bird_json.iter().map(|bird| Bird::from(bird.clone())).collect();
    
    // Loops until the user decides to exist
    loop {
        // Welcome screen, what the user sees when they run the program, options are listed below
        println!("Welcome to the Bird Collection!");
        println!("###############################");
        println!("(a) Addes a bird");
        println!("(p) Prints out all birds");
        println!("(c) prints out all classes");
        println!("(t) Adds a bird to a class");
        println!("(k) Adds a class");
        println!("(j) Search for a bird by scientific name");
        println!("(m) Searches for a bird by common name");
        println!("(q) Quits");
        // asks the user what operation they want to do
        let operation = ask_question();
        // Lowercases it, then trims it, before matching it for all operation
        match operation.to_lowercase().as_str().trim() {
            "a" => {
                // For adding birds, it will ask the two questions mentioned, then simply push a newly constructed bird into the collection
                println!("Whats the name of the bird do you want to add?");
                let bird_name = ask_question();
                println!("Whats the scientific name of the bird?");
                let bird_common_name = ask_question();
                collection.birds.push(
                    Bird { name: bird_name, commonName: bird_common_name, parentNodes: vec![] }
                );
            },
            "p" => {
                // for printing all the birds, simply loop all of the birds, for printing the final string from the parentNodes, 
                // you use String::from_iter which basically concats a iteratable, I then make a iteratable from the birds parentNodes
                // and get the node names and add a seperator
                for bird in &final_bird_json {
                    println!("name: {}, common name: {}, species: {}", bird.name, bird.commonName, 
                        String::from_iter(
                            bird.parentNodes.iter().map(|node| node.name.clone() + " ")
                        ))
                }
            },
            "c" => {
                // For classifications it is more complicated, since they are within all birds
                // you have to loop over all birds while avoiding duplicate refrences, you can do this by
                // using fold, which keeps track of previously iterated parentNode names (within acc)
                // within fold i made a varible called unique_refrences which will filter unique_classifications for unique refrences
                // using .any as the check, which just checks if there has been any previous matches with anything in acc
                // acc is then added with the unique refences, then returned
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
            "j" => {

            },
            "m" => {

            },
            "q" => break,
            _ => {
                println!("Non-existent operation")
            }
        }
    }
}