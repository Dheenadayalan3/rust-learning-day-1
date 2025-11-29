// WRITE THE LOGIC TO FIRST FILTER ALL ODD VALUES THEN DOUBLE EACH VALUE AND CREATE A NEW VECTOR.

pub fn iter_adapter(v: Vec<i32>)-> Vec<i32>{
    let new_iter = v.iter().filter(|x| *x % 2 != 0).map(|x| x * 2);
    let new_vec = new_iter.collect();

    return new_vec; 
}