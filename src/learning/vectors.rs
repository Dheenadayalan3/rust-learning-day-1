pub fn vectors()-> Vec<i32>{
    let mut vect = Vec::new();

    vect.push(1);
    vect.push(2);
    vect.push(3);

    return vect;
}

pub fn even_filter(vector: &Vec<i32>) -> Vec<i32>{
    let mut new_vec = Vec::new();

    for val in vector{
        if val%2 == 0{
            new_vec.push(*val);
        }
    }
 
    new_vec   //implicit return. rust way (Rust treats the last expression without a semicolon as the return value.)
}


/*
why not using this?
hint: ownership

fn main() {
    // Call `vectors()` and store the returned Vec<i32> in `v`
    // Ownership of the vector stays with `v`
    let v = vectors();

    println!("The vector is {:?}", v);

    // Passing `v` into `even_filter` MOVES ownership into the function
    // You cannot use `v` after this line
    println!("The even vector is {:?}", even_filter(v));
}

pub fn vectors() -> Vec<i32> {
    // Create an empty vector
    let mut vect = Vec::new();

    // Push values into the vector
    vect.push(1);
    vect.push(2);
    vect.push(3);

    // Return the vector — ownership moves to the caller
    vect
}

pub fn even_filter(vector: Vec<i32>) -> Vec<i32> {
    // Create a new vector to store even numbers
    let mut new_vec = Vec::new();

    // Iterate through the *owned* vector (ownership moves into the loop)
    for val in vector {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }

    // Return only even numbers
    new_vec
}
 */