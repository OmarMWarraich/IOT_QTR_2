fn main() {
    let number_list = vec![6,11,9,0,4];
    let result = smallest(&number_list);
    println!("The smallest number is {}", result);
    let char_list = vec! ['z','d','c','e','l'];
    let result = smallest(&char_list);
    println!("The smallest charachter is {}", result);
}

fn smallest<T:PartialOrd + Copy> (list : &[T]) -> T {
    let mut smallest = list [0];
    for &item in list.iter() {
        if item < smallest {
            smallest = item;
        }
    }
    smallest
}