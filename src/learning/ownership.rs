pub fn create_string(){
    let a1 = String::from("Dheena");
    let a2 = a1; // Ownership moved from a1 → a2

    println!("Ownership");
    // println!("{}", a1); // Not allowed, a1 is invalid now
    println!("a2 is {}", a2);

    let a1 = a2; // Ownership moved from a2 → a NEW binding a1
    println!("a1 is {}", a1);

    /*
        Key point:
        - This does NOT let you "access original a1".
        - Ownership is just moving around; you never have two owners.
        
        If you want both values:
        1. Use clone — makes a real copy, but costs memory
        2. Borrowing (&) is the most memory-efficient, because no cloning happens
    */


    // Borrowing example — most memory-efficient
    let b1 = String::from("Dheena");
    let b2 = &b1; // Borrow, no ownership move

    println!("Borrowing");
    println!("b1 is {}", b1); 
    println!("b2 is {}", b2); 


    /*
        try this called "Move"

        pub fn create_string() {
            let mut a1 = String::from("Dheena");
            let a2 = a1; // move a1 → a2

            println!("Ownership");
            println!("a2 is {}", a2);

            a1 = a2; // move a2 → existing a1 (old a1 was invalid)
            println!("a1 is {}", a1);

            // Borrowing — no moves
            let b1 = String::from("Dheena");
            let b2 = &b1;

            println!("Borrowing");
            println!("b1 is {}", b1);
            println!("b2 is {}", b2);
        }

        | Feature                              |  `a1 = a2;` |  `let a1 = a2;`            |
| ------------------------------------ | --------------------- | ------------------------------------ |
| Requires `mut`                       | ✔ Yes                 | ❌ No                                 |
| Creates new binding                  | ❌ No                  | ✔ Yes                                |
| Drops old `a1`?                      | ✔ Yes                 | Done automatically when shadowed     |
| Type can change?                     | ❌ No                  | ✔ Yes (shadowing allows type change) |
| Is this modifying existing variable? | ✔ Yes                 | ❌ No (new variable)                  |
| Ownership move                       | ✔ Yes                 | ✔ Yes                                |

    Which should you use?

        Use shadowing when:

            You want a new variable

            You want to change type

            You want to avoid mut

        Use assignment when:

            You want to reuse the same variable

            You need mutation semantics

            You want to overwrite the previous value
    
     */
}
