// // Removing Duplicating by Extracting a Function

// fn main() {
//     let number_list = vec![34,50,25,85,60];

//     let mut largest = number_list[0];

//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     println!("The largest number is {}", largest);
// }


//     let number_list = vec![40,50,60,90,80];
//     let mut largest = number_list[0];

//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }
//     println!("The largest number is {}", largest);
// }

// When we use a parameter in the body of the function, we have to declare the parameter 
// name in the signature so the compiler knows what that name means.
// Place type name declarations inside angle brackets, <>.
// This fn has one parameter named list, which is a slice of values of type T.

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let char_list = largest(&char_list);
//     println!("The largest number is {}", result);
// }

// Gives Following Error:

// "T" might need a bound for std::cmp::PartialOrd

// // Defining a Point struct where x and y are both generics but could have different types.

// struct Point<T,U>  {
//     x : T,
//     y : U,
// }

// fn main() {
//     let both_integer = Point { x:5, y:10 };
//     let both_float = Point { x:1.0, y:2.0 };
//     let integer_and_float = Point { x: 5, y: 4.0}
// }

// // Enum Definitions 
//     enum Option<T>  {
//         Some(T),
//         None,
//     }

// enum Result<T,E) {
//     ok(T),
//     Err(E)
// }

// Here T was filled with the type std::fs::File
// And E was filled with the type std::io::Error

// // Method Definitions

// // Point<T> struct with a method name x called on it

// struct Point<T> {
//     x : T,
//     y: T,
// }
//  implt<T> Point<T> {
//      fn x(&self) -> &T {
//          &self.x
//      }
//  }

//  fn main() {
//      let p = Point { x:5, y:6 };
//      println!("p.x ={}", p.x());
//  }

// //  Here we have defined a method named x on the Point<T>
// //  struct that will return a reference to the date in the field x.
// //  Note that we have to declare T just after impl

// // impl only on Point<f32>
// // Now we do not declare any types after impl 

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// // Following is a method mixup on the Point<T,U> struct from above. This method takes another Point as a Parameter which might
// // have different types from the self Point we are calling mixup on. The method creates a new point instance with the same
// // x value from the self Point(of type T) and y value from the passed-in Point(of type w)

// struct Point<T,U> { 
//     x:T,
//     y:U,
// }

// impl<T, U> Point<T,U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point { 
//             x: self.x,
//             y: self.y,
//         }
//     }
// }

// fn main() {
//     let p1= Point { x:5, y: 10.4 };
//     let p2= Point { x:"Hello", y= 'c'};
//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y ={}", p3.x, p3.y);
// }
// // Above a method that uses different generic types from its struct's definition.
// // In main, Point defined has an i32 for x (with value 5) and an f64 for y (with value 10.4)
// // The p2 variable is a Point struct that has a string slice for x ( with value "Hello")
// // and a char for y(with value C)
// // Calling mixup on p1 with the argument p2 gives us p3, which will have an i32 for x, coz x came from p1.
// // The p3 variable will have a char for y because y came from p2.
// // IThe println! macro call will print p3.x = 5, p3.y = c

// // Monomorphization

// // During the process, the compiler reads the values that have been used in Option<T>






















































// Parameter Type <T>


