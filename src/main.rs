use std::{error::Error, fmt::Display};

struct Classification {
    name: String,
    parent_classification: Box<Classification>
}
impl Classification {
    fn new(name: String, parent_classification: Classification) -> Classification {
        Classification {
            name,
            parent_classification: Box::new(parent_classification),
        }
    }
}

struct Bird {
    name: String,
    classifications: Vec<Classification>
}
struct Collection {
    classifications: Vec<Classification>,
    birds: Vec<Bird>
}
impl Collection {
fn add_and_check_for_duplicates_classifications(){

}
fn add_and_check_for_duplicates_bird() -> Option<i64>{
    if true {
        Some(0)
    } else {
        None
    }
}
}


fn main() {
    // let mut classifications: Vec<Classification> = vec![];
    // add_and_check_for_duplicates()

    // classifications.push(Classification::new("animalia".to_owned()));
    // classifications.push(Classification::new("chordata".to_owned()));
    // classifications.push(Classification::new("aves".to_owned()));
    // classifications.push(Classification::new("gruiformes".to_owned()));
    // classifications.push(Classification::new("rallidae".to_owned()));
    // classifications.push(Classification::new("porphyrio".to_owned()));



}
