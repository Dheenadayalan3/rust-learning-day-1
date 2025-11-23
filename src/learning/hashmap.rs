use std::collections::HashMap;

pub fn vector_to_hashmap(vector: Vec<(String, i32)>)-> HashMap<String, i32>{
    let mut new_hasp_map = HashMap::new();
    for (key, value) in vector{
        new_hasp_map.insert(key, value);
    }

    new_hasp_map
}


