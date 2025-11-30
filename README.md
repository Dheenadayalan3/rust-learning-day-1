# Rust Learning Journey

This repository contains all the Rust concepts I learned today,
including:

-   Modules (`mod`)
-   Structs & Methods
-   Enums & Pattern Matching
-   Option`<T>`{=html} Handling
-   Result\<T, E\> and File Reading
-   Folder structure and imports with `use crate::*`

## Features Covered

### ✔ Struct Example

-   `Rect` struct with `area()` method

### ✔ Enum Example

-   `Shape` enum with `Rectangle` and `Circle` variants
-   `calc_area()` function using pattern matching

### ✔ Option Example

-   `find_first_character()` function that returns Option`<i32>`{=html}

### ✔ Result Example (File Reading)

-   `read_file_content()` function for reading files safely

------------------------------------------------------------------------

## How to Run

``` bash
cargo run
```

------------------------------------------------------------------------

## File Structure

    src/
     ├── main.rs
     └── learning/
           ├── mod.rs
           ├── shape.rs
           ├── enumarate.rs
           ├── find_character.rs
           └── read_file.rs

------------------------------------------------------------------------

📘 Day 2 — Rust Learning Progress
🔹 Topics Covered Today
1. Ownership, Moving, Borrowing

Learned how Rust ensures memory safety without garbage collection

Practiced:

&T — shared borrowing

&mut T — mutable borrowing

Moving values and how ownership transfers

Why returning values sometimes creates moves

2. Collections

Introduction to Rust's collection types

Practical usage of Vec<T>

Vectors

Creating vectors

push() to add elements

Returning vectors safely

Ownership when passing vectors to functions

3. Iterators

Understood differences between:

iter() → immutable iteration (&T)

iter_mut() → mutable iteration (&mut T)

into_iter() → consumes the collection (T)

------------------------------------------------------------------

📘 Rust Learning – Day 3

Today I learned a set of powerful Rust concepts, focusing on iterators, generics, string slicing, and traits with default methods.

✅ 1. Iterator Adapters (filter + map + collect)

Goal: Filter odd values → double them → create a new vector.

pub fn iter_adapter(v: Vec<i32>) -> Vec<i32> {
    let new_iter = v.iter()
        .filter(|x| *x % 2 != 0)
        .map(|x| x * 2);

    let new_vec = new_iter.collect();
    new_vec
}

✅ 2. Iterating Over a HashMap

Learned how to iterate over:

🔹 iter() — immutable key-value pairs
🔹 iter_mut() — mutable key-value pairs
use std::collections::HashMap;

pub fn iter_in_hashmap() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 80);
    scores.insert("Bob", 90);
    scores.insert("Dan", 70);

    println!("Iterating over key-value pairs:");
    for (key, value) in scores.iter() {
        println!("{}: {}", key, value);
    }

    println!("\nIterating over mutable key-value pairs:");
    for (key, value) in scores.iter_mut() {
        *value += 10;
        println!("{}: {}", key, value);
    }
}

✅ 3. Generics with Trait Bounds (PartialOrd)

A function that returns the largest value of any comparable type:

pub fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

✅ 4. Finding the First Word in a String (safe slicing)
pub fn find_first_word(word: &str) -> &str {
    let mut space_index = 0;

    for (_, ch) in word.chars().enumerate() {
        if ch == ' ' {
            break;
        }
        space_index += 1;
    }

    &word[..space_index]
}

✅ 5. Traits, Default Methods, and Trait Bounds
Trait with required method
pub trait Summary {
    fn summarize(&self) -> String;
}

Trait with default method
pub trait Fix {
    fn fix(&self) -> String {
        String::from("Hi there from fix")
    }
}

Struct
pub struct User {
    pub name: String,
    pub age: u32,
}

Implement Summary (custom)
Implement Fix (use default)
impl Summary for User {
    fn summarize(&self) -> String {
        format!("User {} is {} years old", self.name, self.age)
    }
}

impl Fix for User {}

Generic function requiring both traits
pub fn notify<T: Summary + Fix>(u: T) {
    println!("{} {}", u.summarize(), u.fix());
}

----------------------------------------------------------------------

## 📅 Day 4 — Rust Learning Log

### 🔹 1. Lifetimes  
Implemented functions like `longest<'a>` to understand how Rust prevents dangling references.  
Also explored structs with lifetime annotations:

```rust
pub struct User<'a, 'b> {
    first_name: &'a str,
    last_name: &'b str,
}

This ensures both string references live long enough for the struct.

🔹 2. Generics + Traits

Created a function using both lifetimes and trait bounds:

pub fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
where T: Display


Demonstrates combining generics, traits, and lifetimes in one function.

🔹 3. Message Passing with mpsc

Learned to safely send data between threads:

let (tx, rx) = mpsc::channel();
tx.send("Hi").expect("send failed");


Avoided unwrap() and used pattern matching for safer error handling.

🔹 4. Multithreading

Created threads using:

thread::spawn(|| {...});


Also explored:

move closures (transferring ownership across threads)

.join() to wait for thread completion

🔹 5. Full Example

Practiced combining lifetimes, structs, threading, message passing in a single file.