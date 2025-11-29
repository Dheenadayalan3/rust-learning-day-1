
pub fn largest<T: std::cmp::PartialOrd>(a:T, b:T) -> T {
    if a > b {
        a
    } else {
        b
    }
    
}