//  To understand Option<T>

pub fn find_first_character(s: String, character: char)-> Option<i32>{
    for (index, char) in s.chars().enumerate(){
        if char == character {
            return Some(index as i32);
        }
    }



    return None;
}