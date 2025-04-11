use std::{cell::RefCell, collections::HashMap, hash::Hash, io::{stdin, stdout, Write}, vec};
use std::fs;

/// A function to ask the question, error handle, and trim it
fn ask_question() -> String {
    // Constructs a new string for the awnser to be put into
    let mut awnser = String::new();
    // Flush stdout so previous input is not detected
    stdout().flush().unwrap();
    // Reads the actual line (from intput) and also provide error handling
    if let Err(error) = stdin().read_line(&mut awnser) {
        println!("Error: {error}");
    }
    println!("");
    awnser.trim().to_string()
}

// Derive macros so i can compare classifications with other ones, debug it, and clone the data
// to avoid memory issues (since preformance loss is acceptable), there is Hash and Eq which I needed to add so I can access the hashmap methods
#[derive(PartialEq, Hash, Eq, Clone, Debug)]
/// Classification will take a simple string as a name, and a optional, parent classification which is 
/// optional for the sake of orphan nodes and when a node is first initilized
/// and a box for classification so the data can be consistently borrowed as mutable
struct Classification {
    name: String,
    parent_classification: Option<Box<Classification>>
}

impl Classification {
    fn new(name: String, parent_classification: Option<Classification>) -> Classification {
        Classification {
            name,
            parent_classification: {
                if parent_classification.is_none(){
                    None
                } else {
                    Some(Box::new(parent_classification.unwrap()))
                }
            }
        }
    }
}

/// RawBird will be what will be immediately derived from the json file
/// as deserialization will not be possible for parent_nodes which is a property of Bird with the properties I want
/// as in, a classification which is supposed to be a parent node, will have a name and a optional refrence to other parent nodes
/// which is not possible with raw json. 
#[derive(serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct RawBird {
    name: String,
    common_name: String,
    parent_nodes: Vec<String>
}


impl From<RawBird> for Bird {
    // here is a From(RawBird) implimentation to convert the raw json struct into a normal bird struct
    // with the only diffrence being that parent nodes will be turned from simple strings into classifications
    fn from(raw: RawBird) -> Self {
        Bird {
            name: raw.name,
            common_name: raw.common_name,
            parent_nodes: raw.parent_nodes
                .iter()
                .map(|item| RefCell::new(Classification {
                     name: item.to_string(), parent_classification: None
                })).collect(),
        }
    }
}

/// Macros for PartialEq and Debug for equating a bird with another and printing it out, and Clone for avoiding borrowing issues
#[derive(PartialEq, Clone)]
struct Bird {
    name: String,
    common_name: String,
    parent_nodes: Vec<RefCell<Classification>>
}

impl Bird {
    fn new(name: String, parent_nodes: Vec<RefCell<Classification>>, common_name: String) -> Bird {
        Bird {
            name, 
            parent_nodes,
            common_name,
        }
    }
}

/// Collection is everything contained in a single struct, this makes it easier to refer to the data
/// and to better originize certain functions
#[derive(Clone)]
struct Collection {
    classifications: Vec<RefCell<Classification>>,
    classification_tree: HashMap<usize, Vec<Classification>>,
    birds: Vec<Bird>
}
/// The implimentation for Collection which will provide a constructer function, while also providing functions
/// for ensuring there are no duplicate classifications, or birds, and one where it will add a classification if none exists,
/// and return the refrence (regardless if it existed prior or not)
impl Collection {
    fn new() -> Collection {
        Collection { classifications: vec![], birds: vec![], classification_tree: HashMap::new()  }
    }
    // This searches the birds by species, 
    fn search_bird_by_species(&self, name: String) -> Vec<&Bird> {
        let mut matched_birds = vec![];
        let mut used_names = std::collections::HashSet::new();
        let mut nodes: Vec<String> = vec![name.clone()];
    
        while let Some(current_name) = nodes.pop() {
            if !used_names.insert(current_name.clone()) {
                continue;
            }
    
            for classifications in self.classification_tree.values() {
                for node in classifications {
                    if let Some(parent) = &node.parent_classification {
                        if parent.name == current_name {
                            nodes.push(node.name.clone());
                        }
                    }
                }
            }
        }
    
        for bird in &self.birds {
            for classification in &bird.parent_nodes {
                let name = classification.borrow().name.clone();
                if used_names.contains(&name) {
                    matched_birds.push(bird);
                    break;
                }
            }
        }
    
        matched_birds
    }
    
    fn search_bird_by_name(&self, name: String) -> Vec<&Bird> {
        let mut matched_birds = vec![];
        for bird in &self.birds {
            if bird.name.contains(&name){
                matched_birds.push(bird);
            }
        }
        matched_birds
    }
    fn search_bird_by_common_name(&self, name: String) -> Vec<&Bird> {
        let mut matched_birds = vec![];
        for bird in &self.birds {
           println!("{}, {}", bird.name, name);
            if bird.common_name.contains(&name){
                matched_birds.push(bird);
            }
        }
        matched_birds
    }
    fn add_and_check_for_duplicates_classifications<'a>(&'a mut self, classification: &Classification) -> Option<usize> {
        if !self.classifications.iter().any(|c| c.borrow().name == classification.name) {
            self.classifications.push(RefCell::new(classification.clone()));
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
    fn print_classifications(self){
        // For classifications it is more complicated, since they are within all birds
        // you have to loop over all birds while avoiding duplicate refrences, you can do this by
        // using fold, which keeps track of previously iterated parentNode names (within acc)
        // within fold i made a varible called unique_refrences which will filter unique_classifications for unique refrences
        // using .any as the check, which just checks if there has been any previous matches with anything in acc
        // acc is then added with the unique refences, then returned
        for classification in self.birds.iter().fold(
            vec![], |mut acc, bird| {
                let unique_refrences: Vec<Classification> = bird.parent_nodes
                    .iter()
                    .map(|unique_classification| unique_classification.borrow().clone())
                    .filter(|unique_classificaion|
                        !acc.contains(unique_classificaion)
                        //c.iter().any(|matching_classification: &Classification| matching_classification == unique_classificaion)
                        )
                        .collect();
                    acc.extend(unique_refrences);
                    acc
            }).into_iter().map(|classification| classification.name.clone()){
            println!("{}", classification)
        }
    }
}
/// This is by far the most complicated function in all the code and one that took awhile to make, this orginizes 
/// classifications by frequency, by making a few assumtions that seems to be accurate about the data. Parent Nodes are not arbitrary
/// Meaning that nodes are bigger in heirarchy if they are more frequently mentioned in all the birds, Second is that there can only be one parent for a child node
/// which i retrive the order and current name from. (order and amount is frequency).
fn sort_by_classification(birds: Vec<Bird>) -> HashMap<usize, Vec<Classification>> {
    // The first thing i do is make a HashMap with a String, which will store the name of classifications and intergers, which will be used per classification to determine how
    // often it appears in all the birds.
    let mut acc: HashMap<String, usize> = HashMap::new();

    // I then loop over the birds, then classifications, to count how many times they appear and increment its value in the hashmap
    for bird in &birds {
        for classification in &bird.parent_nodes {
            let name = classification.borrow().name.clone();
            *acc.entry(name).or_insert(0) += 1;
        }
    }

    // then I loop over all the birds agin but this time i get all the names of all the classifications of a bird in a loop, then go over the birds parent nodes.
    for bird in &birds {
        let parent_names: Vec<String> = bird
            .parent_nodes
            .iter()
            .map(|c| c.borrow().name.clone())
            .collect();

        for classification_rc in &bird.parent_nodes {
            let current_name = classification_rc.borrow().name.clone();
            let order = acc.get(&current_name).copied().unwrap_or(0);

            // Then under potential_parents I look for nodes refrenced more than the current one, and if that
            // node is in the same array as it in a bird (will not work if they never are in the same array), 
            // to ensure that there are no bad groupings in a single heirarchy by ensuring that a single heirarchy
            // covers the bird that is supposed to be a child of it.
            let potential_parents: Vec<Classification> = bird
                .parent_nodes
                .iter()
                .filter_map(|c| {
                    let borrowed = c.borrow();
                    let amount = acc.get(&borrowed.name).copied().unwrap_or(0);
                    if amount > order && parent_names.contains(&borrowed.name) {
                        Some(borrowed.clone())
                    } else {
                        None
                    }
                })
                .collect();
            // Then I simply re-arrange it in the final loop to flip the key and value, and put the actual classifications instead of just a string
            // and group them so i can get all the nodes at a certain place in the heirachy
            let mut classification = classification_rc.borrow_mut();
            for parent in potential_parents {
                classification.parent_classification = Some(Box::new(parent));
            }
        }
    }

    let mut final_node_tree: HashMap<usize, Vec<Classification>> = HashMap::new();
    for bird in birds {
        for classification_refcell in bird.parent_nodes {
            let classification = classification_refcell.borrow();
            let amount = acc.get(&classification.name).copied().unwrap_or(0);
            final_node_tree.entry(amount).or_default().push(classification.clone());
        }
    }

    final_node_tree
}


/// Main function, it will run all my code, first thing it does is that it initilizes the collection
/// Then using fs it will read the birdData json file, (with error handling), captured as a string
/// next it is deserilized into Vec<RawBird> (vector of rawbirds) because rawbirds is more accurate to the JSON data
/// then it is converted into a final struct, by iterating over all the birds gained from the json file.
/// using Bird::from it will convert it into regular birds 
fn main() {
    let mut collection = Collection::new();
    let raw_bird_data: String  = fs::read_to_string("birdData.json").expect("Failed");
    let raw_bird_json: Vec<RawBird> = serde_json::from_str(&raw_bird_data).expect("Incorrect json");
    let final_bird_json: Vec<Bird> = raw_bird_json.iter().map(|bird| Bird::from(bird.clone())).collect();
    collection.classification_tree = sort_by_classification(final_bird_json.clone());
    
    collection.birds.extend(final_bird_json.clone());


    // Loops until the user decides to exist
    loop {
        // Welcome screen, what the user sees when they run the program, options are listed below
        println!("Welcome to the Bird Collection!");
        println!("###############################");
        println!("(a) Addes a bird");
        println!("(p) Prints out all birds");
        println!("(c) prints out all classes");
        println!("(k) Adds a class");
        println!("(j) Search for a bird by name");
        println!("(m) Searches for a bird by common name");
        println!("(l) Searches for a bird by species");
        println!("(q) Quits");
        // asks the user what operation they want to do
        let operation = ask_question();
        // Lowercases it, then trims it, before matching it for all operation
        match operation.to_lowercase().as_str().trim() {
            "a" => {
                // For adding birds, it will ask the two questions mentioned, then simply push a newly constructed bird into the collection
                // First thing it does it check if the bird name if under 50 characters long
                let mut bird_name: String;
                loop {
                    println!("Whats the name of the bird do you want to add?");
                    bird_name = ask_question();
                    if bird_name.len() > 50 {
                        println!("Name cannot be longer than 50 characters");
                        continue
                    }
                    break
                }
                println!("Whats the common name of the bird?");
                let bird_common_name = ask_question();

                let bird_to_add = collection.add_and_check_for_duplicates_bird(Bird::new(bird_name, vec![], bird_common_name));
                if bird_to_add.is_none() {
                    println!("That bird already exists")
                }
                
            },
            "p" => {
                // for printing all the birds, simply loop all of the birds, for printing the final string from the parent_nodes, 
                // you use String::from_iter which basically concats a iteratable, I then make a iteratable from the birds parent_nodes
                // and get the node names and add a seperator
                for bird in &final_bird_json {
                    println!("name: {}, common name: {}, species: {}", bird.name, bird.common_name, 
                        String::from_iter(
                            bird.parent_nodes.iter().map(|node| node.borrow().name.clone() + " ")
                        ))
                }
            },
            "c" => {
                // I moved this into a function as I use it elsewhere in code and it reduces boilerplate
                collection.clone().print_classifications();
            },
            "k" => {
                println!("Whats the name of the species you want to add? (taxaomic grouping");
                let species_name = ask_question();
                let species_to_add = collection.add_and_check_for_duplicates_classifications(&Classification::new(species_name, None));
                if species_to_add.is_none() {
                    println!("Error: Duplicate species detected")
                }
            },
            "j" => {
                println!("What is the name (it will work with partial input)");
                let name = ask_question();
                let birds = collection.search_bird_by_name(name);
                for bird in birds.iter() {
                    println!("name: {}, common name: {}, species: {}", bird.name, bird.common_name, 
                        String::from_iter(
                            bird.parent_nodes.iter().map(|node| node.borrow().name.clone() + " ")
                        ))
                }
            },
            "m" => {
                println!("What is the common name (it will work with partial input)");
                let name = ask_question();
                let birds = collection.search_bird_by_common_name(name);
                println!("Matched birds:");
                for bird in &birds {
                    println!("name: {}, common name: {}, species: {}", bird.name, bird.common_name, 
                        String::from_iter(
                            bird.parent_nodes.iter().map(|node| node.borrow().name.clone() + " ")
                        ))
                }
            },
            "l" => {
                println!("What species are you searching for? (full species name required)");
                let name = ask_question();
                let birds = collection.search_bird_by_species(name);
                for bird in &birds {
                    println!("name: {}, common name: {}, species: {}", bird.name, bird.common_name, 
                        String::from_iter(
                            bird.parent_nodes.iter().map(|node| node.borrow().name.clone() + " ")
                        ))
                }
            }
            "q" => break,
            _ => {
                println!("Non-existent operation")
            }
        }
    }
}