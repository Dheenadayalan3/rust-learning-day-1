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

