
// Preventing Dangling references with lifetimes.


{
    let r;

    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}

// error. value out of scope. x dropped while borrowed

// Generic Lifetimes in Funcitons

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest result is: {}", result);
}

// Lifetime Annotation Syntax

& i32 // a reference to
&'a i32 // a reference with explicit lifetime
&'a muti32 // a mutable reference with explicit lifetime

// Lifetime Annotations in Function Sugnatures

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime annotations restrict the longest function by passing in references 
// having different concrete lifetimes

fn main() {
    let string1 = String::from("lkbhlkhlkj");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}


// Thinking in Terms of lifetimes

fn longest<'a>(x: &'a str, y: str) -> &'a str {
    
}

// Lifetime Annotations in Struct Definitions

//ImportantExcerpt

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me..");
    let first_sentence = novel.split('.').next().expect("Cudnt");;
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// Lifetime Elision

// no lifetimes associated with references

fn first_word(s: &str) -> &str {
}

//
fn first_word<a'>(s: &'s str) -> & str
//\\
fn first_word<'a>(s: &'a str) -> &str

fn longest(x: &str, y: &str) -> &str

fn longest<'a,'b>(x: &'a str, y: &'b str) -> &struct


// Lifetime Annot in Methods
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str{
        println!("Attention please: {:?}", announcement);
        self.part`
    }
}\

// The Static Lifetime

let s: &'static str = " Ihave a static lifetime";

// Generic Type Pzarameters, Trait Bounds and Lifetimes Together

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str, 
    y: &'a str,
    ann: T,)-> &'a str {
        where T: Display,
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                y
            }
        }

        
    


















































































