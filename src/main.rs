mod learning;

use crate::learning::shape::Rect;
use crate::learning::enumarate::{calc_area, Shape};
use crate::learning::find_character::find_first_character;
use crate::learning::read_file::read_file_content;

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
}