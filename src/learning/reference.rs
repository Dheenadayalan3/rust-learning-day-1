pub fn reference() {
    // a1 owns the String
    let a1 = String::from("Hello");

    // Immutable borrow of a1
    let a2 = &a1;

    // Another immutable borrow of a1 — allowed
    let a3 = &a1;

    // You can have any number of immutable references
    // because they do not allow modifying the data.
    println!(
        "Borrowing: a1 is {}, a2 is {}, a3 is {}",
        a1, a2, a3
    );
}

pub fn mutable_reference() {
    // a1 owns the String and must be mutable
    let mut a1 = String::from("Hello");

    println!("Before mutation, a1 is {}", a1);

    // Create a mutable reference to a1
    let a2 = &mut a1;

    // Cannot borrow a1 immutably here:
    // let a3 = &a1;          //  ERROR: cannot borrow `a1` as immutable because it is already borrowed as mutable.
                              // At any given time, you can have either one mutable reference or any number of immutable references.
 
    // With a mutable reference, you can change the value
    a2.push_str(", world!");
    println!("a2 is {}", a2);
    // After a mutable borrow is fully done (after a2 last used),
    // a1 becomes usable again.

    // Using a2 ends here because we no longer access it.
    println!("After mutation, a1 is {}", a1);
    //println!("a2 is {}", a2); cannot use it here
}
