use std::{sync::mpsc, thread::{ spawn }};

pub fn message_passing(){
    let ( tx, rx ) = mpsc::channel();
    spawn(move || {
             // Here you can use unwarp but if error comes rust panics and good codebase shouldn't use unwrap (Remember cloudfare outage)
        match tx.send("Hi from send") {
            Ok(_) => println!("Send successfully"),
            Err(err) => println!("{err}"),
        }
    });

    let val = rx.recv(); // Here you can use unwarp but if error comes rust panics and good codebase shouldn't use unwrap (Remember cloudfare outage)
    match val {
        Ok(val) => println!("Recieved from sender : {val}"),
        Err(err) => println!("{err}"),
    }
}
