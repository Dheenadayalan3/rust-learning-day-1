use std::fmt::Display;


pub struct User<'a, 'b> {
    first_name: &'a str,
    last_name: &'b str,
}

pub fn structs_lifetime(){
    let user : User;
    println!("Example from structs with lifetime ");
    let first_name = String::from("Naruto ");
    {
        let second_name = String::from("Uzumaki");
        user = User {
            first_name : &first_name,
            last_name : &second_name,
        };
        println!("The name of the user is {}{}!", user.first_name, user.last_name);
    }
    
    //println!("The name of the user is {}", user.first_name); //`second_name` does not live long enough
}

pub fn longest_with_an_announcement<'a, T>(x: &'a str,y: &'a str,ann: T) -> &'a str where T:Display {
    println!("Announcement {ann}");
    if x.len() > y.len(){
        x
    } else {
        y
    }
}