use std::collections::{HashMap};

pub fn iter_in_hashmap(){
    let mut scores = HashMap::new();
    scores.insert("Alice", 80);
    scores.insert("Bob", 90);
    scores.insert("Dan", 70);

    // Ex:01 => Iterating over references to key-value pairs
    println!("Iterating over key-value pairs: ");
    for (key, value) in scores.iter(){
        println!("{}: {} ", key, value);
    }

    //Ex:02 => Iterating over mutable references to key-value pairs
    println!("\nIterating over mutable key-value pairs: ");
    for (key, value) in scores.iter_mut(){
        *value = *value + 10;
        println!("{}: {} ", key, value);
    }
    
}