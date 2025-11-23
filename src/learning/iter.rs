pub fn iteration(){
    let mut vector1 = vec![1,2,3];
    println!("Before vector mutation : {:?}",vector1);

    let mut vector1_iter = vector1.iter_mut();

    while let Some(value) = vector1_iter.next(){
        *value = *value + 1;
    }

    /*
    for value in vector1_iter{
        *value = *value + 1; 
    }
     */
    

    println!("After vector mutation : {:?}",vector1);
}

/*
| Method         | Item type | Ownership | Can modify?      | Vector still usable? |
| -------------- | --------- | --------- | ---------------- | -------------------- |
| `.iter()`      | `&T`      | No        | ❌ No            | ✔ Yes                |
| `.iter_mut()`  | `&mut T`  | No        | ✔ Yes            | ✔ Yes                |
| `.into_iter()` | `T`       | ✔ Yes     | N/A (owns items) | ❌ No                |

*/