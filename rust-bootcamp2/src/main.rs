// fn main() {
//     println!("Hello, world!");
// }


/*
===================================================
================= Variables (nums, strings, bools)
================= Conditionals
================= Loops
================== Functions
===================================================
*/



// Que: Check if a number is even.
// fn main () {
//     let ans = is_even(20);
//     println!("{}", ans);
// }
// // i  means signed (i32), and u means unsigned (u32)
// fn is_even (num: i64) -> bool {
//     if num%2 == 0 {
//         return true;
//     } else {
//         return false;
//     }
// }





// que: Write a function fib that finds the fibbonacci of a number it takes as input.
// fn main() {
//     println!("{}", fib(10));
// }

// fn fib (num: u32) -> u32 { 
//     let mut first = 0;
//     let mut second = 1;

//     if num == 0 {
//         return first;
//     }    

//     if num == 1 {
//         return second;
//     }

//     for _ in 0..(num - 2) {
//         let temp = second;
//         second = second + first;
//         first = temp;
//     }
//     return second;
// }


// Que: Write a function get_string_length that takes a string as an input returns its length.
// fn main() {
//     let name = String::from("my name is applexity");
//     let len = get_string_length(name);
//     println!("The length of the string is {}", len);
// }
// fn get_string_length(str: String) -> usize {
//     str.chars().count()
// }





// # Understanding Structs

// struct User {
//     first_name: String,
//     last_name: String,
//     age: i32,
// }
// fn main() {
//     let user = User {
//         first_name: String::from("Applexity"),
//         last_name: String::from("Ox"),
//         age: 20,
//     };

//     println!(" First Name : {} Last Name: {} Age: {}" , user.first_name, user.last_name, user.age);
// }









// Implementing a struct



// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect = Rect {
//         width: 30,
//         height: 50,
//     };
//     println!("The area of the rectangle is {}", rect.area());
// }












// Structs lets you structure data together.

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("Applexity"),
//         email: String::from("applexity@gmail.com"),
//         sign_in_count: 1,
//     };
//     print!("User 1 username: {:?}", user1.username);
//     print!("User 1 active: {:?}", user1.active);
//     print!("User 1 email: {:?}", user1.email);
//     print!("User 1 sign_in_count: {:?}", user1.sign_in_count);
// }















// Enums let you enumerate over various types of a value.

// enum Direction {
//     North,
//     East,
//     West,
//     South,
// }

// fn main() {
//     let my_direction = Direction::North;
//     let new_direction = my_direction;
//     move_around(new_direction);
// }

// fn new_direction(direction: Direction) {
//     // implement logic to move a character around
// }







// Option Enum (For Null Values in Rust)

// fn find_first_a(s: String) -> Option <i32> {
//     for (index, character) in s.chars().enumerate() {
//         if character === 'a' {
//             return Some(index as i32);
//         }
//     }
//     return None;
// }

// fn main() {
//     let my_string = String::from("applexity");
//     match find_first_a(my_string) {
//             Some(index) => println!("The letter 'a' is found at index: {}", index);
//             None => print!("The letter 'a' is not found in the string.");
//     }
// }














// Results
// Write a funtion that reads the contents of a file.

// use std::fs;

// fn main() {
//     let greeting_file_result = fs::read_to_string("a.txt"); // Result

//     match greeting_file_result {
//         OK(file_content) => {
//             println!("File read successfully: {:?}", file_content);
//         },
//         Err(error) => {
//             println!("Failed to read file: {:?}", error);
//         } 
//     }
// }





// Trait Implementation:

// trait _Greet {
//     fn say_hello(&self);
// }


// struct _Person {
//     name: String,
// }

// // Implement the Greet trait for the Person struct
// impl _Greet for _Person {
//     fn say_hello(&self) {
//         println!("Hello, my name is {}", self.name);
//     }
// }











// Matching on enums

/*

fn mina() {
    println("hi");
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Variant can hold data
}

enum UsState {
    Alabama,
    Alaska,
    // ... other states
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // Bind the inner value to a variable 'state'
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

*/










/*
====================
Package management
===================
*/ 



// 

// External packages/crate
// > cargo init <crate_name>

// creading current time.

use chrono::Local;
fn main() {
    let now = Local::now();
    println!("Current Time is {}", now);
}





















