mod learning;

use crate::learning::generic::largest;
use crate::learning::shape::Rect;
use crate::learning::enumarate::{calc_area, Shape};
use crate::learning::find_character::find_first_character;
use crate::learning::read_file::read_file_content;
use crate::learning::ownership::create_string;
use crate::learning::reference::{reference, mutable_reference};
use crate::learning::slice::find_first_word;
use crate::learning::traits::{Summary, User, notify};
use crate::learning::vectors::{even_filter, vectors};
use crate::learning::hashmap::vector_to_hashmap;
use crate::learning::iter::{iteration};
use crate::learning::iter_adapt::iter_adapter;
use crate::learning::iter_hashmap::iter_in_hashmap;

fn main(){

    // Create a Rect struct instance
    let r: Rect = Rect{
        width: 23, 
        height: 789
    };

    println!("Area is {}", r.area());

    //-------------------------------------------------------------------------------------

    // Create enum variants of Shape
    let circle: Shape = Shape::Circle(67.8);
    let rectangle = Shape::Rectangle(234.3, 2.1);

    println!("The area of circle is {}", calc_area(circle));
    println!("The area of rectangle is {}", calc_area(rectangle));

    //--------------------------------------------------------------------------------------

    // find_first_character returns Option<i32>, so handle both Some and None
    let name = String::from("Preet");
    let index = find_first_character(name, 'a'); 

    // Unwrapping the Option using pattern matching
    match index{
        Some(index) => println!("The index is {}",index), 
        None => println!("The character is not found"),
    }
    
    // Result concept
    match read_file_content("data.txt") {
        Ok(text) => println!("File content:\n{}", text),
        Err(e) => println!("Error reading file: {}", e),
    }

    // Ownership
    create_string();

    //Reference
    reference();
    mutable_reference();

    //Vectors
    let v = vectors();

    println!("The vector is {:?}",v);
    println!("The even vector is {:?}",even_filter(&v));

    //HashMap
    let input_vector = vec![(String::from("Dan"), 1), (String::from("Sam"),2)];

    println!("The hashmap is {:#?}",vector_to_hashmap(input_vector));

    // Iteration
    iteration();

    //Adapter 
    let v1 = vec![1, 2, 3, 4, 5, 6];
    let ans = iter_adapter(v1);

    println!("{:?}", ans);

    //Hashmap iteration
    iter_in_hashmap();

    //slice
    println!("Slice: The first word is {}",find_first_word("Hello Rust"));

    // Genrics
    println!("The largest among is {}",largest(10, 100));
    println!("The largest among is {}",largest('a', 'd'));
    println!("The largest among is {}",largest("aydyt", "sfhilwef"));

    // Traits
    let user = User{
        name: String::from("Dan"),
        age: 23,
    };

    println!("{}", user.summarize()); // direct call
    notify(user); // this will print summarize + fix
}