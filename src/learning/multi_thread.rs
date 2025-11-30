use std::thread;

pub fn multi_threading(){
    println!("Examples from multithreading");
    let handle = thread::spawn( || {                        // || this notation is borrow closure
            for i in 1..5 {
                println!("The variable is from spawned thread {}", i);
            }
        }
        
    );

    for i in 1..10 {
        println!("The variable is from main thread {}", i);
    };

    handle.join().unwrap();

}

pub fn multi_thread_move(){

    let x = 1;

    {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {               //  'move ||' takes ownership
            println!("{:?}", v);
        });
        handle.join().unwrap();
    }
    println!("{}", x);

}