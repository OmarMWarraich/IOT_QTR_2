use std::thread;
use std::time::Duration;

fn main() {
    let score = 2;
    let add_2 = || {score + 2};
    for i in 0..10 {
        println!("add_2: {}", add_2());
    }
    let goal=2;
    let multiply_goal = thread::spawn(move ||{
        for j in 0..15{
            println!("multiply_goal: {}",goal*2);
            thread::sleep(Duration::from_millis(20));
        }
    });
    multiply_goal.join().unwrap();
   
}

