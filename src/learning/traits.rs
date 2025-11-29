// -------------------------------------------------------
// Trait Summary: Requires any type to provide a custom
// textual summary about itself.
// -------------------------------------------------------
pub trait Summary {
    fn summarize(&self) -> String;
}

// -------------------------------------------------------
// Trait Fix: Demonstrates default method implementations.
// Any type that implements Fix will automatically get
// the `fix()` method unless it overrides it.
// -------------------------------------------------------
pub trait Fix {
    // Default implementation: can be used as-is
    fn fix(&self) -> String {
        String::from("Hi there from fix")
    }
}

// -------------------------------------------------------
// Struct User: A simple data type with two fields.
// -------------------------------------------------------
pub struct User {
    pub name: String,
    pub age: u32,
}

// -------------------------------------------------------
// Implementation of Summary for User:
// We MUST define summarize(), because Summary does not
// provide a default implementation.
// -------------------------------------------------------
impl Summary for User {
    fn summarize(&self) -> String {
        format!("User {} is {} years old", self.name, self.age)
    }
}

// -------------------------------------------------------
// Implementation of Fix for User:
//
// Note: We don't implement any method here because the
// Fix trait *already provides a default method*.
// Rust automatically uses it. If need also we can override
// -------------------------------------------------------
impl Fix for User {}

// -------------------------------------------------------
// Generic function with trait bounds:
// notify<T: Summary + Fix>
//
// This means: notify() can accept ANY type T that
// implements BOTH Summary AND Fix.
//
// Inside, we call summarize() and fix() safely, because
// the trait bounds guarantee that these methods exist.
// -------------------------------------------------------
pub fn notify<T: Summary + Fix>(u: T) {
    println!("{} {}", u.summarize(), u.fix());
}