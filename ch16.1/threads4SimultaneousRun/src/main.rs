// Creaating a new thread with spawn

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi numer {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi numer {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// Waiting for all threads to finish using join Handles

use std::thread;
use std::time::Duration;

fn main() {

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi numer {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi numer {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

// Using handle.join() before for loop in above example.


use std::thread;
use std::time::Duration;

fn main() {

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi numer {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi numer {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    
}

// main thread shall run after handle

// Using move closures with threads

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here a vector: {:?}", v );
        
    });
    handle.join().unwrap();
}

// Rust doesnt know how long thread runs so it doesnt know ref to v shall be valid always


// Another failed scenario

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here a vector: {:?}", v );
        
    });
    drop(v); //oh no 
    handle.join().unwrap();
}
// move


use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here a vector: {:?}", v );
        
    });
    handle.join().unwrap();
}































