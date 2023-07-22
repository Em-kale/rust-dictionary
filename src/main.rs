use std::io; 
use serde_json::Value;
use std::io::Read;


fn main() {
    /* all variables in rust are immutable unless you use the mut keyword */
    let mut word = String::new();

    println!("Enter word: ");

    /* could be written as std::io::stdin() this function returns an instance of the stdin type */
    let input = io::stdin();


    input.read_line(&mut word).expect("failed to read input");

    //returns an instance of request::blocking::Response

    //trimmed_word is now a &str, which is a reference to a string slice (str) 
    let trimmed_word = word.trim();
  

    //in order to concatenate, you might need to reallocate if there isn't space
    //&str is just a reference to a string slice though, if we want to allocate a string such that it has
    //the base + the word, we need a String (which owns its own data and can allocate space for more). 
    let endpoint = "https://api.dictionaryapi.dev/api/v2/entries/en/".to_owned() + trimmed_word;
    let mut definition_result = reqwest::blocking::get(&endpoint).expect("Unable to open");  
    let mut result = String::new(); 

    //request::blocking::Response provides a function called read_to_string()
    //this function is implemented from the read trait, so that must be included in the scope
    definition_result.read_to_string(&mut result).expect("failed to read to string");

    //serde json provides a datatype for storing json
    //serde_json provides value type 
    let definition_information: Value = serde_json::from_str(&result).expect("failed to convert from json"); 
    
  
    let mut num_of_definitions: i32 = 1; 
    for definition in definition_information[0]["meanings"][0]["definitions"].as_array().expect("No definitions found"){
        println!("\nDefinition {}: \n", num_of_definitions);
        println!(" {}", definition["definition"]);
        num_of_definitions = num_of_definitions + 1; 
     }
}


   