
fn main(){
    let score = 2;
    let add_2 = || {score + 2};
    for i in 0..10 {
        println!("add_2: {}",add_2());
    }
}


